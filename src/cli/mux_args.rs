//! Command-line arguments for persistent network mux sessions.
//!
//! [`MuxArgs`] wraps one [`MuxCommand`] such as creating, attaching to,
//! listing, or stopping a named server.

#[path = "mux_args/command.rs"]
mod mux_command;

use clap::Parser;
pub use mux_command::MuxCommand;

/// Network mux lifecycle arguments.
///
/// # Examples
///
/// ```
/// use codetether_agent::cli::command::mux_args::{MuxArgs, MuxCommand};
/// let args = MuxArgs { command: MuxCommand::List { json: false } };
/// assert!(matches!(args.command, MuxCommand::List { json: false }));
/// ```
#[derive(Debug, Parser)]
pub struct MuxArgs {
    /// Mux lifecycle operation to perform.
    #[command(subcommand)]
    pub command: MuxCommand,
}

#[cfg(test)]
#[path = "mux_args_tests.rs"]
mod tests;

#[cfg(test)]
#[path = "mux_args_kill_tests.rs"]
mod kill_tests;

#[cfg(test)]
#[path = "mux_args_help_tests.rs"]
mod help_tests;
