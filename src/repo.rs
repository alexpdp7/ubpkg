use include_dir::{include_dir, Dir};
use thiserror::Error;

pub const REPO: Dir = include_dir!("repo");

#[derive(Error, Debug)]
pub enum RepoError {
    #[error("Manifest {0:?} not found")]
    NotFound(String),
    #[error("Malformed UTF8 in manifest {0:?}")]
    MalformedUTF8(String),
}

pub struct Manifest {
    pub name: String,
    pub content: String,
}

pub fn load_manifest_from_repo(package: String) -> Result<Manifest, RepoError> {
    let manifest_name = format!("{}.ubpkg.sky", package);
    Ok(Manifest {
        name: manifest_name.clone(),
        content: REPO
            .get_file(&manifest_name)
            .ok_or_else(|| RepoError::NotFound(manifest_name.clone()))?
            .contents_utf8()
            .ok_or_else(|| RepoError::MalformedUTF8(manifest_name.clone()))?
            .to_string(),
    })
}
