//! Worktree command-line schema, dispatch and operation reporting.
//!
//! [WorktreeArgs] selects a command; [run] dispatches it using the shared
//! worktree manager. Cleanup is a preview unless explicitly applied.

mod cleanup;
mod cleanup_report;
mod list;
mod open;
mod workspace;

use codetether_worktree::WorktreeManager;

/// Dispatch a worktree subcommand against the current repository.
///
/// # Arguments
/// * `args` - Parsed operation, output format and safety options.
/// # Returns
/// Unit after the requested operation and its output complete.
/// # Errors
/// Returns repository, filesystem, serialization or editor-launch errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// codetether_worktree_cli::run(codetether_worktree_cli::WorktreeArgs {
///     command: codetether_worktree_cli::WorktreeCommand::List(
///         codetether_worktree_cli::WorktreeListArgs { json: true }),
/// }).await
/// # }
/// ```
pub async fn run(args: WorktreeArgs) -> anyhow::Result<()> {
    let mgr = WorktreeManager::for_repo(std::env::current_dir()?);
    match args.command {
        WorktreeCommand::Cleanup(a) => cleanup::run(a, &mgr).await,
        WorktreeCommand::List(a) => list::run(a, &mgr).await,
        WorktreeCommand::Open(a) => open::run(a, &mgr).await,
        WorktreeCommand::Workspace(a) => workspace::run(a, &mgr).await,
    }
}

mod args;
mod args_cleanup;
mod args_list;
mod args_open;
mod args_workspace;
pub use args::{WorktreeArgs, WorktreeCommand};
pub use args_cleanup::WorktreeCleanupArgs;
pub use args_list::WorktreeListArgs;
pub use args_open::WorktreeOpenArgs;
pub use args_workspace::WorktreeWorkspaceArgs;
