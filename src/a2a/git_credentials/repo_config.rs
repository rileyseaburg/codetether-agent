//! Compatibility exports for repository-owned credential configuration.
#[cfg(test)]
pub(super) use codetether_worktree::repository::credentials::run_git_config_command_with_lock_recovery;
pub use codetether_worktree::repository::credentials::{
    configure_repo_git_auth, configure_repo_git_github_app,
};
