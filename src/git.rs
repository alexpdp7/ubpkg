use starlark::environment::GlobalsBuilder;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error executing git (maybe git is not installed?)")]
    GitExecution(#[from] std::io::Error),
    #[error("Return code {0} when executing {1}")]
    GitUnsuccessful(std::process::ExitStatus, String),
}

#[allow(clippy::missing_panics_doc)]
pub fn get_repo_sorted_versions(url: String) -> Result<Vec<String>, Error> {
    let mut binding = std::process::Command::new("git");
    let command = binding
        .arg("ls-remote")
        .arg("--tags")
        .arg("--refs")
        .arg(url)
        .stderr(std::process::Stdio::inherit());
    let refs = command.output()?;

    if !refs.status.success() {
        return Err(Error::GitUnsuccessful(refs.status, format!("{command:?}")));
    }

    let output = String::from_utf8(refs.stdout).unwrap();
    let mut versions = output
        .split('\n')
        .filter_map(|l| l.split_once('\t')) // the last line is blank :(
        .map(|(_hash, id)| id)
        .map(|id| id.strip_prefix("refs/tags/").unwrap().to_string())
        .collect::<Vec<String>>();

    versions.sort_by(|a: &String, b: &String| vsort::compare(a, b));
    Ok(versions)
}

#[allow(clippy::missing_panics_doc)]
pub fn get_repo_latest_version(url: String) -> Result<String, Error> {
    Ok(get_repo_sorted_versions(url)?.last().unwrap().clone())
}

#[starlark_module]
pub fn git(builder: &mut GlobalsBuilder) {
    fn get_repo_latest_version(url: &str) -> anyhow::Result<String> {
        Ok(get_repo_latest_version(url.to_string())?)
    }
}
