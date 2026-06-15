use clap::{Parser, Subcommand};
use std::path::PathBuf;

/// Manage git worktrees and their VS Code integration.
#[derive(Parser, Debug)]
pub struct WorktreeArgs {
    #[command(subcommand)]
    pub command: WorktreeCommand,
}

#[derive(Subcommand, Debug)]
pub enum WorktreeCommand {
    /// List worktrees discovered for the current repository
    List(WorktreeListArgs),

    /// Open a worktree in VS Code
    Open(WorktreeOpenArgs),

    /// Write a multi-root `.code-workspace` covering the repo and worktrees
    Workspace(WorktreeWorkspaceArgs),
}

#[derive(Parser, Debug)]
pub struct WorktreeListArgs {
    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}

#[derive(Parser, Debug)]
pub struct WorktreeOpenArgs {
    /// Worktree name (as shown by `worktree list`)
    pub name: String,

    /// Reuse the current VS Code window instead of opening a new one
    #[arg(short, long)]
    pub reuse_window: bool,
}

#[derive(Parser, Debug)]
pub struct WorktreeWorkspaceArgs {
    /// Destination path for the `.code-workspace` file
    #[arg(long, default_value = "codetether.code-workspace")]
    pub dest: PathBuf,

    /// Open the generated workspace in VS Code after writing it
    #[arg(long)]
    pub open: bool,
}
