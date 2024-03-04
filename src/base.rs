use std::io::Read;

use allocative::Allocative;
use starlark::environment::GlobalsBuilder;
use starlark::starlark_simple_value;
use starlark::values::{
    starlark_value, NoSerialize, ProvidesStaticType, StarlarkValue, Value, ValueLike,
};
use std::os::unix::fs::PermissionsExt;

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

#[starlark_value(type = "file_contents")]
impl<'v> StarlarkValue<'v> for FileContents {}

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
        let mut perms = std::fs::metadata(dest).unwrap().permissions();
        perms.set_mode(perms.mode() | 0o100);
        std::fs::set_permissions(dest, perms).unwrap();
        Ok(0)
    }

    fn download_asset(url: &str, max_size: u64) -> anyhow::Result<FileContents> {
        let resp = ureq::get(url).call()?;
        assert!(resp.has("Content-Length"));
        let len: usize = resp.header("Content-Length").unwrap().parse()?;

        let mut bytes: Vec<u8> = Vec::with_capacity(len);
        resp.into_reader().take(max_size).read_to_end(&mut bytes)?;

        Ok(FileContents { contents: bytes })
    }

    fn extract_from_url(url: &str, path: &str) -> anyhow::Result<FileContents> {
        let path = path.to_string();
        let tmp_dir = tempdir::TempDir::new("ubpkg").unwrap();
        let mut extracted_path = tmp_dir.path().to_path_buf();
        extracted_path.push(path.clone());
        let mut asset_path = tmp_dir.path().to_path_buf();
        asset_path.push(url.split('/').last().unwrap());
        std::io::copy(
            &mut ureq::get(url).call()?.into_reader(),
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
            contents: std::fs::read(extracted_path).unwrap(),
        })
    }
}
