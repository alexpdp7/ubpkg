#![allow(
    clippy::needless_lifetimes,
    clippy::unnecessary_wraps,
    clippy::elidable_lifetime_names
)] // starlark weirdness

use std::io::Read;

use allocative::Allocative;
use anyhow::Context;
use starlark::environment::{GlobalsBuilder, Methods, MethodsBuilder, MethodsStatic};
use starlark::starlark_simple_value;
use starlark::values::{
    starlark_value, NoSerialize, ProvidesStaticType, StarlarkValue, Value, ValueLike,
};

#[derive(Debug, ProvidesStaticType, NoSerialize, Allocative)]
struct FileContents {
    contents: Vec<u8>,
}
starlark_simple_value!(FileContents);

impl std::fmt::Display for FileContents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "filecontents")
    }
}

#[starlark_module]
fn file_contents_methods(builder: &mut MethodsBuilder) {
    fn as_string(#[starlark(this)] receiver: Value) -> anyhow::Result<String> {
        let file_contents = receiver.downcast_ref::<FileContents>().unwrap();
        Ok(String::from_utf8(file_contents.contents.clone())?)
    }
}

#[starlark_value(type = "file_contents")]
impl<'v> StarlarkValue<'v> for FileContents {
    fn get_methods() -> Option<&'static Methods> {
        static RES: MethodsStatic = MethodsStatic::new();
        RES.methods(file_contents_methods)
    }
}

#[starlark_module]
pub fn base(builder: &mut GlobalsBuilder) {
    fn install_binary(contents: Value, name: &str) -> anyhow::Result<i32> {
        let contents = contents.downcast_ref::<FileContents>().unwrap();
        let mut dest = directories::UserDirs::new()
            .unwrap()
            .home_dir()
            .to_path_buf();
        dest.push(".local");
        dest.push("bin");
        dest.push(name);
        let dest = dest.as_path();
        std::fs::write(dest, contents.contents.clone())?;
        #[cfg(not(windows))]
        {
            use std::os::unix::fs::PermissionsExt;

            let mut perms = std::fs::metadata(dest).unwrap().permissions();
            perms.set_mode(perms.mode() | 0o100);
            std::fs::set_permissions(dest, perms).unwrap();
        }
        Ok(0)
    }

    fn download_asset(url: &str, max_size: u64) -> anyhow::Result<FileContents> {
        let mut resp = ureq::get(url)
            .call()
            .with_context(|| format!("Error getting {url}"))?;
        assert!(resp.headers().contains_key("Content-Length"));
        let len: usize = resp
            .headers()
            .get("Content-Length")
            .unwrap()
            .to_str()
            .unwrap()
            .parse()?;

        let mut bytes: Vec<u8> = Vec::with_capacity(len);
        resp.body_mut()
            .as_reader()
            .take(max_size)
            .read_to_end(&mut bytes)?;

        Ok(FileContents { contents: bytes })
    }

    fn extract_from_url(url: &str, path: &str) -> anyhow::Result<FileContents> {
        let path = path.to_string();
        let tmp_dir = tempfile::TempDir::new().unwrap();
        let mut extracted_path = tmp_dir.path().to_path_buf();
        extracted_path.push(path.clone());
        let mut asset_path = tmp_dir.path().to_path_buf();
        let mut request = ureq::get(url)
            .call()
            .with_context(|| format!("Error getting {url}"))?;
        let attachment_prefix = "attachment; filename=";
        if let Some(content_disposition) = request
            .headers()
            .get("content-disposition")
            .map(|c| c.to_str().unwrap())
        {
            asset_path.push(content_disposition.strip_prefix(attachment_prefix).unwrap());
        } else {
            asset_path.push(url.split('/').next_back().unwrap());
        }
        std::io::copy(
            &mut request.body_mut().as_reader(),
            &mut std::fs::File::create(asset_path.clone())?,
        )?;
        let x = extracted_path.clone();
        decompress::decompress(
            asset_path,
            tmp_dir.path().to_path_buf(),
            &decompress::ExtractOptsBuilder::default()
                .filter(move |p| p == x) // produces output https://github.com/rusty-ferris-club/decompress/issues/15
                .build()
                .unwrap(),
        )
        .unwrap();

        Ok(FileContents {
            contents: std::fs::read(&extracted_path).with_context(|| {
                format!("{} not found in archive at {url}", extracted_path.display())
            })?,
        })
    }

    fn fail(message: &str) -> anyhow::Result<i32> {
        anyhow::bail!("{}", message.to_owned());
    }
}
