use include_dir::{include_dir, Dir};
use starlark::environment::{GlobalsBuilder, Module};
use starlark::eval::Evaluator;
use starlark::syntax::AstModule;
use starlark::syntax::Dialect;
use starlark::values::{NoSerialize, Value};
#[macro_use]
extern crate starlark;
use crate::starlark::values::ValueLike;
use allocative::Allocative;
use starlark::environment::Methods;
use starlark::environment::MethodsBuilder;
use starlark::environment::MethodsStatic;
use starlark::values::starlark_value;
use starlark::values::AllocFrozenValue;
use starlark::values::AllocValue;
use starlark::values::FrozenHeap;
use starlark::values::FrozenValue;
use starlark::values::Heap;
use starlark::values::ProvidesStaticType;
use starlark::values::StarlarkValue;

const REPO: Dir = include_dir!("repo");

fn main() {
    let package = "vale";
    let manifest_name = format!("{}.ubpkg.sky", package);
    let manifest = REPO
        .get_file(manifest_name.clone())
        .unwrap()
        .contents_utf8()
        .unwrap();

    let ast: AstModule = AstModule::parse(
        &manifest_name.clone(),
        manifest.to_owned(),
        &Dialect::Extended,
    )
    .unwrap();

    let mut globals = GlobalsBuilder::new().with(github).with(base);
    globals.set("version", starlark::values::none::NoneType);
    globals.set("os", std::env::consts::OS);
    globals.set("arch", std::env::consts::ARCH);
    let globals = globals.build();
    let module: Module = Module::new();
    let mut eval: Evaluator = Evaluator::new(&module);
    let res: Value = eval.eval_module(ast, &globals).unwrap();
    println!("{:?}", res);
}

#[derive(Debug, ProvidesStaticType, NoSerialize, Allocative, Clone)]
struct GitHubRepo {
    id: String,
}

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

impl<'v> AllocValue<'v> for GitHubRepo {
    fn alloc_value(self, heap: &'v Heap) -> Value<'v> {
        heap.alloc_simple(self)
    }
}

impl AllocFrozenValue for GitHubRepo {
    fn alloc_frozen_value(self, heap: &FrozenHeap) -> FrozenValue {
        heap.alloc_simple(self)
    }
}

#[derive(Debug, ProvidesStaticType, NoSerialize, Allocative, Clone)]
struct GitHubRelease {
    github_repo: GitHubRepo,
}

impl std::fmt::Display for GitHubRelease {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "gh:{}", self.github_repo)
    }
}

#[starlark_module]
fn release_methods(builder: &mut MethodsBuilder) {
    fn name(#[starlark(this)] receiver: Value) -> anyhow::Result<String> {
        let release = receiver.downcast_ref::<GitHubRelease>().unwrap();
        Ok("foo".to_string())
    }

    fn download_asset(
        #[starlark(this)] receiver: Value,
        name: &str,
    ) -> anyhow::Result<GitHubAsset> {
        let release = receiver.downcast_ref::<GitHubRelease>().unwrap();
        Ok(GitHubAsset {
            github_release: release.clone(),
            name: name.to_string(),
        })
    }
}

#[starlark_value(type = "github_release")]
impl<'v> StarlarkValue<'v> for GitHubRelease {
    fn get_methods() -> Option<&'static Methods> {
        static RES: MethodsStatic = MethodsStatic::new();
        RES.methods(release_methods)
    }
}

impl<'v> AllocValue<'v> for GitHubRelease {
    fn alloc_value(self, heap: &'v Heap) -> Value<'v> {
        heap.alloc_simple(self)
    }
}

impl AllocFrozenValue for GitHubRelease {
    fn alloc_frozen_value(self, heap: &FrozenHeap) -> FrozenValue {
        heap.alloc_simple(self)
    }
}

#[derive(Debug, ProvidesStaticType, NoSerialize, Allocative)]
struct GitHubAsset {
    github_release: GitHubRelease,
    name: String,
}

impl std::fmt::Display for GitHubAsset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "release:{} name:{}", self.github_release, self.name)
    }
}

#[starlark_value(type = "github_release")]
impl<'v> StarlarkValue<'v> for GitHubAsset {}

impl<'v> AllocValue<'v> for GitHubAsset {
    fn alloc_value(self, heap: &'v Heap) -> Value<'v> {
        heap.alloc_simple(self)
    }
}

impl AllocFrozenValue for GitHubAsset {
    fn alloc_frozen_value(self, heap: &FrozenHeap) -> FrozenValue {
        heap.alloc_simple(self)
    }
}

#[derive(Debug, ProvidesStaticType, NoSerialize, Allocative)]
struct FileContents {}

impl std::fmt::Display for FileContents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "filecontents")
    }
}

#[starlark_value(type = "file_contents")]
impl<'v> StarlarkValue<'v> for FileContents {}

impl<'v> AllocValue<'v> for FileContents {
    fn alloc_value(self, heap: &'v Heap) -> Value<'v> {
        heap.alloc_simple(self)
    }
}

impl AllocFrozenValue for FileContents {
    fn alloc_frozen_value(self, heap: &FrozenHeap) -> FrozenValue {
        heap.alloc_simple(self)
    }
}

#[starlark_module]
fn github(builder: &mut GlobalsBuilder) {
    fn github_repo(repo: &str) -> anyhow::Result<GitHubRepo> {
        Ok(GitHubRepo {
            id: repo.to_string(),
        })
    }
}

#[starlark_module]
fn base(builder: &mut GlobalsBuilder) {
    fn install_binary(contents: Value, name: &str) -> anyhow::Result<i32> {
        Ok(0)
    }
    fn extract(asset: Value, archive_format: &str, path: &str) -> anyhow::Result<FileContents> {
        Ok(FileContents {})
    }
}
