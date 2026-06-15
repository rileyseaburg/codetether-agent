//! `codetether worktree` command handlers (list / open / workspace).

mod list;
mod open;
mod workspace;

use codetether_agent::{
    cli::{WorktreeArgs, WorktreeCommand},
    worktree::WorktreeManager,
};

/// Dispatch a `worktree` subcommand against the current repository.
pub async fn run(args: WorktreeArgs) -> anyhow::Result<()> {
    let mgr = WorktreeManager::for_repo(std::env::current_dir()?);
    match args.command {
        WorktreeCommand::List(a) => list::run(a, &mgr).await,
        WorktreeCommand::Open(a) => open::run(a, &mgr).await,
        WorktreeCommand::Workspace(a) => workspace::run(a, &mgr).await,
    }
}
