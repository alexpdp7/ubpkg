use clap::Parser;
use color_eyre::eyre::Result;

use ubpkg::{repo, runner};

#[derive(Parser)]
#[command()]
struct Args {
    #[arg(short, long)]
    version: Option<String>,
    packages: Vec<String>,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let args = Args::parse();
    for package in &args.packages {
        let manifest = repo::load_manifest_from_repo(package)?;
        runner::run_manifest(manifest, args.version.clone())?;
    }
    Ok(())
}
