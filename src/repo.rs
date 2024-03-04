use include_dir::{include_dir, Dir};
use thiserror::Error;

pub const REPO: Dir = include_dir!("repo");

#[derive(Error, Debug)]
pub enum Error {
    #[error("Manifest {0:?} not found")]
    NotFound(String),
    #[error("Malformed UTF8 in manifest {0:?}")]
    MalformedUTF8(String),
}

pub struct Manifest {
    pub name: String,
    pub content: String,
}

#[allow(clippy::module_name_repetitions)]
pub fn load_manifest_from_repo(package: &str) -> Result<Manifest, Error> {
    let manifest_name = format!("{package}.ubpkg.sky");
    Ok(Manifest {
        name: manifest_name.clone(),
        content: REPO
            .get_file(&manifest_name)
            .ok_or_else(|| Error::NotFound(manifest_name.clone()))?
            .contents_utf8()
            .ok_or_else(|| Error::MalformedUTF8(manifest_name.clone()))?
            .to_string(),
    })
}
