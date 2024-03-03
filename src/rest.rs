use crate::starlark::values::ValueLike;
use allocative::Allocative;
use starlark::environment::GlobalsBuilder;
use starlark::environment::Methods;
use starlark::environment::MethodsBuilder;
use starlark::environment::MethodsStatic;
use starlark::starlark_simple_value;
use starlark::values::starlark_value;
use starlark::values::ProvidesStaticType;
use starlark::values::StarlarkValue;
use starlark::values::{NoSerialize, Value};
use std::os::unix::fs::PermissionsExt;

#[derive(Debug, ProvidesStaticType, NoSerialize, Allocative, Clone)]
struct GitHubRepo {
    id: String,
}
starlark_simple_value!(GitHubRepo);

impl std::fmt::Display for GitHubRepo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "gh:{}", self.id)
    }
}

#[starlark_module]
fn repo_methods(builder: &mut MethodsBuilder) {
    fn latest_release(#[starlark(this)] receiver: Value) -> anyhow::Result<GitHubRelease> {
        let repo = receiver.downcast_ref::<GitHubRepo>().unwrap();

        let refs = std::process::Command::new("git")
            .arg("ls-remote")
            .arg("--tags")
            .arg("--refs")
            .arg(format!("https://github.com/{}.git", repo.id))
            .stderr(std::process::Stdio::inherit())
            .output()
            .unwrap();

        assert!(refs.status.success());

        let output = String::from_utf8(refs.stdout).unwrap();
        let mut versions = output
            .split('\n')
            .flat_map(|l| l.split_once('\t')) // the last line is blank :(
            .map(|(_hash, id)| id)
            .map(|id| id.strip_prefix("refs/tags/").unwrap())
            .collect::<Vec<&str>>();

        versions.sort_by(|a: &&str, b: &&str| vsort::compare(a, b));

        Ok(GitHubRelease {
            github_repo: repo.clone(),
            tag: versions.last().unwrap().to_string(),
        })
    }
}

#[starlark_value(type = "github_repo")]
impl<'v> StarlarkValue<'v> for GitHubRepo {
    fn get_methods() -> Option<&'static Methods> {
        static RES: MethodsStatic = MethodsStatic::new();
        RES.methods(repo_methods)
    }
}

#[derive(Debug, ProvidesStaticType, NoSerialize, Allocative, Clone)]
struct GitHubRelease {
    github_repo: GitHubRepo,
    tag: String,
}
starlark_simple_value!(GitHubRelease);

impl std::fmt::Display for GitHubRelease {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "gh:{}", self.github_repo)
    }
}

#[starlark_module]
fn release_methods(builder: &mut MethodsBuilder) {
    fn name(#[starlark(this)] receiver: Value) -> anyhow::Result<String> {
        let release = receiver.downcast_ref::<GitHubRelease>().unwrap();
        Ok(release.tag.clone())
    }

    fn get_asset_url(#[starlark(this)] receiver: Value, name: &str) -> anyhow::Result<String> {
        let release = receiver.downcast_ref::<GitHubRelease>().unwrap();
        Ok(format!(
            "https://github.com/{}/releases/download/{}/{}",
            release.github_repo.id, release.tag, name,
        ))
    }
}

#[starlark_value(type = "github_release")]
impl<'v> StarlarkValue<'v> for GitHubRelease {
    fn get_methods() -> Option<&'static Methods> {
        static RES: MethodsStatic = MethodsStatic::new();
        RES.methods(release_methods)
    }
}

#[derive(Debug, ProvidesStaticType, NoSerialize, Allocative)]
struct GitHubAsset {
    github_release: GitHubRelease,
    name: String,
}
starlark_simple_value!(GitHubAsset);

impl std::fmt::Display for GitHubAsset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "release:{} name:{}", self.github_release, self.name)
    }
}

#[starlark_value(type = "github_release")]
impl<'v> StarlarkValue<'v> for GitHubAsset {}

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
pub fn github(builder: &mut GlobalsBuilder) {
    fn github_repo(repo: &str) -> anyhow::Result<GitHubRepo> {
        Ok(GitHubRepo {
            id: repo.to_string(),
        })
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
