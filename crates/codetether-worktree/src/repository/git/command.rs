//! Repository Git command operation.
use super::*;

/// Run a git command at an optional working directory.
///
/// # Arguments
/// * `current_dir` - Working directory, or the process directory when absent.
/// * `args` - Git arguments passed directly without a shell.
/// # Returns
/// Trimmed standard output from a successful Git process.
/// # Errors
/// Returns an error when Git cannot start or reports a nonzero exit status.
/// # Examples
/// ```rust,no_run
/// # async fn example() -> anyhow::Result<()> {
/// use codetether_worktree::repository::git::run_git_command_at;
/// let output = run_git_command_at(None, vec!["--version".into()]).await?;
/// assert!(output.starts_with("git version"));
/// # Ok(()) }
/// ```
pub async fn run_git_command_at(current_dir: Option<&Path>, args: Vec<String>) -> Result<String> {
    let mut command = Command::new("git");
    if let Some(dir) = current_dir {
        command.current_dir(dir);
    }
    let output = command
        .args(args.iter().map(String::as_str))
        .output()
        .await
        .context("Failed to execute git command")?;
    if output.status.success() {
        return Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
    }
    Err(anyhow::anyhow!(
        "Git command failed: {}",
        String::from_utf8_lossy(&output.stderr).trim()
    ))
}
