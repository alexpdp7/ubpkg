use crate::starlark::values::ValueLike;
use allocative::Allocative;
use starlark::environment::GlobalsBuilder;
use starlark::starlark_simple_value;
use starlark::values::starlark_value;
use starlark::values::ProvidesStaticType;
use starlark::values::StarlarkValue;
use starlark::values::{NoSerialize, Value};
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
        std::fs::write(dest, contents.contents.clone()).unwrap();
        let mut perms = std::fs::metadata(dest).unwrap().permissions();
        perms.set_mode(perms.mode() | 0o100);
        std::fs::set_permissions(dest, perms).unwrap();
        Ok(0)
    }
    fn extract_from_url(url: &str, path: &str) -> anyhow::Result<FileContents> {
        let path = path.to_string();
        let tmp_dir = tempdir::TempDir::new("ubpkg").unwrap();
        let mut extracted_path = tmp_dir.path().to_path_buf();
        extracted_path.push(path.clone());
        let mut asset_path = tmp_dir.path().to_path_buf();
        asset_path.push(url.split('/').last().unwrap());
        std::fs::write(
            asset_path.clone(),
            &reqwest::blocking::get(url).unwrap().bytes().unwrap(),
        )
        .unwrap();
        let x = extracted_path.clone();
        decompress::decompress(
            asset_path,
            tmp_dir.path().to_path_buf(),
            &decompress::ExtractOptsBuilder::default()
                .filter(move |p| p == x)
                .build()
                .unwrap(),
        )
        .unwrap();

        Ok(FileContents {
            contents: std::fs::read(extracted_path).unwrap(),
        })
    }
}
