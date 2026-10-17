//! Credential configuration operation: run git command.
use super::*;

/// Runs a Git command inside a repository and surfaces stderr on failure.
///
/// The command is executed with the repository as the current directory.
///
/// # Examples
///
/// ```ignore
/// run_git_command(repo_path, &["status", "--short"])?;
/// ```
pub fn run_git_command(repo_path: &Path, args: &[&str]) -> Result<()> {
    let output = Command::new("git")
        .current_dir(repo_path)
        .args(args)
        .output()?;
    if output.status.success() {
        return Ok(());
    }
    Err(anyhow!(
        "Git command failed in {}: {}",
        repo_path.display(),
        String::from_utf8_lossy(&output.stderr).trim()
    ))
}
