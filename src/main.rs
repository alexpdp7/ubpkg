use color_eyre::eyre::Result;

use ubpkg::{repo, runner};

fn main() -> Result<()> {
    color_eyre::install()?;
    let manifest = repo::load_manifest_from_repo("vale".to_string())?;
    runner::run_manifest(manifest)?;
    Ok(())
}
