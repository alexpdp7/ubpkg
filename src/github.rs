use allocative::Allocative;
use starlark::environment::{Methods, MethodsBuilder, MethodsStatic};
use starlark::values::{
    starlark_value, NoSerialize, ProvidesStaticType, StarlarkValue, Value, ValueLike,
};

use crate::git;

#[derive(Debug, ProvidesStaticType, NoSerialize, Allocative, Clone)]
pub struct GitHubRepo {
    pub id: String,
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
        Ok(GitHubRelease {
            github_repo: repo.clone(),
            tag: git::get_repo_latest_version(format!("https://github.com/{}.git", repo.id))?,
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
