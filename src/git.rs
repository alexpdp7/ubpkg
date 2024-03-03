use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitError {
    #[error("Error executing git (maybe git is not installed?)")]
    GitExecution(#[from] std::io::Error),
    #[error("Return code {0} when executing {1}")]
    GitUnsuccessful(std::process::ExitStatus, String),
}

pub fn get_repo_sorted_versions(url: String) -> Result<Vec<String>, GitError> {
    let mut binding = std::process::Command::new("git");
    let command = binding
        .arg("ls-remote")
        .arg("--tags")
        .arg("--refs")
        .arg(url)
        .stderr(std::process::Stdio::inherit());
    let refs = command.output()?;

    if !refs.status.success() {
        return Err(GitError::GitUnsuccessful(
            refs.status,
            format!("{command:?}"),
        ));
    }

    let output = String::from_utf8(refs.stdout).unwrap();
    let mut versions = output
        .split('\n')
        .flat_map(|l| l.split_once('\t')) // the last line is blank :(
        .map(|(_hash, id)| id)
        .map(|id| id.strip_prefix("refs/tags/").unwrap().to_string())
        .collect::<Vec<String>>();

    versions.sort_by(|a: &String, b: &String| vsort::compare(a, b));
    Ok(versions)
}

pub fn get_repo_latest_version(url: String) -> Result<String, GitError> {
    Ok(get_repo_sorted_versions(url)?.last().unwrap().to_string())
}
