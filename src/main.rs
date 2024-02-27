use include_dir::{include_dir, Dir};
use starlark::environment::{GlobalsBuilder, Module};
use starlark::eval::Evaluator;
use starlark::syntax::AstModule;
use starlark::syntax::Dialect;
use starlark::values::{NoSerialize, Value};
#[macro_use]
extern crate starlark;
use allocative::Allocative;
use starlark::values::starlark_value;
use starlark::values::AllocValue;
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

#[derive(Debug, ProvidesStaticType, NoSerialize, Allocative)]
struct GitHubRepo {}

impl std::fmt::Display for GitHubRepo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "gh")
    }
}

#[starlark_value(type = "github_repo")]
impl<'v> StarlarkValue<'v> for GitHubRepo {}

impl<'v> AllocValue<'v> for GitHubRepo {
    fn alloc_value(self, heap: &'v Heap) -> Value<'v> {
        heap.alloc_simple(self)
    }
}

#[starlark_module]
fn github(builder: &mut GlobalsBuilder) {
    fn github_repo(repo: &str) -> anyhow::Result<GitHubRepo> {
        Ok(GitHubRepo {})
    }
}

#[starlark_module]
fn base(builder: &mut GlobalsBuilder) {
    fn install_binary(a: &str, b: &str, c: &str) -> anyhow::Result<i32> {
        Ok(0)
    }
    fn extract(a: &str, b: &str) -> anyhow::Result<i32> {
        Ok(0)
    }
}
