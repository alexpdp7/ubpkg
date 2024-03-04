use clap::Parser;
use color_eyre::eyre::Result;

use ubpkg::{repo, runner};

#[derive(Parser)]
#[command()]
struct Args {
    packages: Vec<String>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    for package in args.packages.iter() {
        let manifest = repo::load_manifest_from_repo(package.to_string())?;
        runner::run_manifest(manifest)?;
    }
    Ok(())
}
