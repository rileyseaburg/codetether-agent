//! Worktree command arguments for listing, opening, and safely cleaning checkouts.
//!
//! [`WorktreeCommand::Cleanup`] defaults to a read-only preview and requires
//! `--apply` before it mutates Git worktree registrations.

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

    /// Preview or remove clean worktrees already merged into a base branch
    Cleanup(WorktreeCleanupArgs),
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

/// Options for repository-wide safe worktree cleanup.
#[derive(Parser, Debug)]
pub struct WorktreeCleanupArgs {
    /// Branch or commit that candidates must already be merged into.
    #[arg(long, default_value = "main")]
    pub base: String,
    /// Limit cleanup to worktrees beneath one or more directories.
    #[arg(long)]
    pub root: Vec<PathBuf>,
    /// Apply cleanup; omission produces a read-only preview.
    #[arg(long)]
    pub apply: bool,
    /// Output the complete cleanup report as JSON.
    #[arg(long)]
    pub json: bool,
}
