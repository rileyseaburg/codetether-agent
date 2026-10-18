//! Command-line arguments for persistent network mux sessions.
//!
//! [`MuxArgs`] wraps one [`MuxCommand`] such as creating, attaching to,
//! listing, or stopping a named server.

use crate::MuxCommand;
use clap::Parser;

/// Network mux lifecycle arguments.
///
/// # Examples
///
/// ```
/// use codetether_mux_cli::{MuxArgs, MuxCommand};
/// let args = MuxArgs { command: MuxCommand::List { json: false } };
/// assert!(matches!(args.command, MuxCommand::List { json: false }));
/// ```
#[derive(Debug, Parser)]
pub struct MuxArgs {
    /// Mux lifecycle operation to perform.
    #[command(subcommand)]
    pub command: MuxCommand,
}
