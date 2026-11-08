//! Monarch operations in the standard authentication command tree.
use clap::{Parser, Subcommand};

/// Monarch OAuth command options; omitting a subcommand starts login.
///
/// # Examples
/// ```
/// use clap::Parser;
/// let args = codetether_auth_cli::MonarchAuthArgs::parse_from(["monarch"]);
/// assert!(args.command.is_none());
/// ```
#[derive(Parser, Debug)]
pub struct MonarchAuthArgs {
    /// Operation; defaults to login.
    #[command(subcommand)]
    pub command: Option<MonarchCommand>,
}

/// Explicit Monarch credential operations.
///
/// # Examples
/// ```
/// let action = codetether_auth_cli::MonarchCommand::Status;
/// assert!(matches!(action, codetether_auth_cli::MonarchCommand::Status));
/// ```
#[derive(Subcommand, Debug)]
pub enum MonarchCommand {
    /// Authorize read-only access through Monarch's official consent page.
    Login,
    /// Check the configured secret backend without exposing credentials.
    Status,
    /// Revoke the remote grant and clear its configured secret record.
    Logout,
}
