//! Persistent, authenticated network multiplexing for CodeTether workspaces.
//!
//! A mux server owns named window metadata independently of terminal clients.
//! Clients can detach and reconnect over loopback TCP without relying on tmux.
//! [`execute`] dispatches the CLI lifecycle operations, while internal model,
//! registry, protocol, client, and server modules each own one subsystem.
//!
//! ## Usage
//!
//! ```text
//! codetether mux new -s work -c /path/to/project -d
//! codetether mux attach -t work
//! codetether mux list
//! codetether mux kill -t work
//! codetether mux kill-all
//! ```

mod client;
mod command;
pub(crate) mod control;
mod model;
mod protocol;
mod pty;
mod registry;
mod server;
mod token;

use crate::cli::command::mux_args::MuxArgs;

/// Execute a network mux lifecycle command.
///
/// # Arguments
///
/// * `args` - Parsed named-session lifecycle operation.
///
/// # Returns
///
/// Returns after the requested server or client lifecycle operation completes.
///
/// # Errors
///
/// Returns an error when validation, persistence, process startup, or network
/// communication fails.
///
/// # Examples
///
/// ```rust,no_run
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// use codetether_agent::cli::command::mux_args::{MuxArgs, MuxCommand};
///
/// let args = MuxArgs { command: MuxCommand::List { json: true } };
/// codetether_agent::mux::execute(args).await.unwrap();
/// # });
/// ```
pub async fn execute(args: MuxArgs) -> anyhow::Result<()> {
    command::execute(args.command).await
}

#[cfg(test)]
mod tests;
