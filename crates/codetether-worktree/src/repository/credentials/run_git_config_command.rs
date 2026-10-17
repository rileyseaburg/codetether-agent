//! Credential configuration operation: run git config command.
use super::*;

pub(super) fn run_git_config_command(repo_path: &Path, args: &[&str]) -> Result<()> {
    run_git_config_command_with_lock_recovery(repo_path, args, Duration::from_secs(30))
}
