//! Context history listing and turn selection.
use clap::{Parser, Subcommand};

/// Optional browse action; omission lists available turns.
/// # Examples
/// ```
/// let args = codetether_context_cli::ContextBrowseArgs { command: None };
/// assert!(args.command.is_none());
/// ```
#[derive(Parser, Debug)]
pub struct ContextBrowseArgs {
    /// Selected subcommand.
    #[command(subcommand)]
    pub command: Option<ContextBrowseCommand>,
}

/// List available turns or show a selected turn.
/// # Examples
/// ```
/// use codetether_context_cli::ContextBrowseCommand;
/// match ContextBrowseCommand::List { ContextBrowseCommand::List => {}, ContextBrowseCommand::ShowTurn(_) => {} }
/// ```
#[derive(Subcommand, Debug)]
pub enum ContextBrowseCommand {
    /// List virtual paths for each turn in the latest session
    List,
    /// Show one turn as markdown-like text
    ShowTurn(ContextShowTurnArgs),
}

/// Zero-based turn to display.
/// # Examples
/// ```
/// let args = codetether_context_cli::ContextShowTurnArgs { turn: 0 };
/// assert_eq!(args.turn, 0);
/// ```
#[derive(Parser, Debug)]
pub struct ContextShowTurnArgs {
    /// Zero-based turn index
    pub turn: usize,
}
