//! Command-line schema for worktree operations.

use crate::{WorktreeCleanupArgs, WorktreeListArgs, WorktreeOpenArgs, WorktreeWorkspaceArgs};
use clap::{Parser, Subcommand};

/// Manage git worktrees and their VS Code integration.
///
/// # Examples
/// ```
/// let args = codetether_worktree_cli::WorktreeArgs {
///     command: codetether_worktree_cli::WorktreeCommand::List(
///         codetether_worktree_cli::WorktreeListArgs { json: false }),
/// };
/// assert!(matches!(args.command, codetether_worktree_cli::WorktreeCommand::List(_)));
/// ```
#[derive(Parser, Debug)]
pub struct WorktreeArgs {
    /// Worktree operation to execute.
    #[command(subcommand)]
    pub command: WorktreeCommand,
}

/// Supported listing, editor, workspace-file and cleanup operations.
///
/// # Examples
/// ```
/// let command = codetether_worktree_cli::WorktreeCommand::List(
///     codetether_worktree_cli::WorktreeListArgs { json: true });
/// match command {
///     codetether_worktree_cli::WorktreeCommand::List(args) => assert!(args.json),
///     _ => unreachable!(),
/// }
/// ```
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
