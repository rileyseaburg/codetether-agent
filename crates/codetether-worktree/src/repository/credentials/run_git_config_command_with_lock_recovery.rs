//! Credential configuration operation: run git config command with lock recovery.
use super::*;

#[cfg_attr(test, allow(dead_code))]
pub fn run_git_config_command_with_lock_recovery(
    repo_path: &Path,
    args: &[&str],
    stale_after: Duration,
) -> Result<()> {
    match run_git_command(repo_path, args) {
        Ok(()) => Ok(()),
        Err(err) if is_git_config_lock_error(&err.to_string()) => {
            remove_stale_git_config_lock(repo_path, stale_after).with_context(|| {
                format!(
                    "Failed to recover stale Git config lock in {}",
                    repo_path.display()
                )
            })?;
            run_git_command(repo_path, args).with_context(|| {
                format!(
                    "Git config command still failed after stale lock recovery in {}",
                    repo_path.display()
                )
            })
        }
        Err(err) => Err(err),
    }
}
