//! Top-level context command selection.
use crate::{ContextBrowseArgs, ContextResetArgs};
use clap::{Parser, Subcommand};

/// Select a context command.
/// # Examples
/// ```
/// use codetether_context_cli::{ContextArgs, ContextCommand, ContextResetArgs};
/// let args = ContextArgs { command: ContextCommand::Reset(ContextResetArgs { summary: None }) };
/// assert!(matches!(args.command, ContextCommand::Reset(_)));
/// ```
#[derive(Parser, Debug)]
pub struct ContextArgs {
    /// Selected subcommand.
    #[command(subcommand)]
    pub command: ContextCommand,
}

/// Manual context reset or history browsing.
/// # Examples
/// ```
/// use codetether_context_cli::{ContextCommand, ContextResetArgs};
/// let command = ContextCommand::Reset(ContextResetArgs { summary: None });
/// match command { ContextCommand::Reset(_) => {}, ContextCommand::Browse(_) => {} }
/// ```
#[derive(Subcommand, Debug)]
pub enum ContextCommand {
    /// Emit a [CONTEXT RESET] marker into the latest session
    Reset(ContextResetArgs),
    /// Browse the latest session as virtual turn files
    Browse(ContextBrowseArgs),
}
