use starlark::environment::{GlobalsBuilder, Module};
use starlark::eval::Evaluator;
use starlark::syntax::{AstModule, Dialect};
use starlark::values::Value;

use thiserror::Error;

use crate::base;
use crate::git;
use crate::github;
use crate::repo::Manifest;

#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("Parsing error: {0}")]
    ParsingError(starlark::Error),
    #[error("Runtime error: {0}")]
    RuntimeError(starlark::Error),
}

pub fn run_manifest(manifest: Manifest) -> Result<(), ExecutionError> {
    let ast: AstModule = AstModule::parse(&manifest.name, manifest.content, &Dialect::Extended)
        .map_err(ExecutionError::ParsingError)?;

    let mut globals = GlobalsBuilder::new()
        .with(github::github)
        .with(base::base)
        .with(git::git);
    globals.set("os", std::env::consts::OS);
    globals.set("arch", std::env::consts::ARCH);
    let globals = globals.build();
    let module: Module = Module::new();
    let mut eval: Evaluator = Evaluator::new(&module);
    let _res: Value = eval
        .eval_module(ast, &globals)
        .map_err(ExecutionError::RuntimeError)?;
    Ok(())
}
