//! Command-line arguments for persistent network mux sessions.
//!
//! [`MuxArgs`] wraps one [`MuxCommand`] such as creating, attaching to,
//! listing, or stopping a named server. Clap uses these types beneath the
//! top-level `codetether mux` command.

use std::net::SocketAddr;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Network mux lifecycle arguments.
///
/// # Examples
///
/// ```
/// use codetether_agent::cli::command::mux_args::{MuxArgs, MuxCommand};
///
/// let args = MuxArgs { command: MuxCommand::List { json: false } };
/// assert!(matches!(args.command, MuxCommand::List { json: false }));
/// ```
#[derive(Debug, Parser)]
pub struct MuxArgs {
    /// Mux lifecycle operation to perform.
    #[command(subcommand)]
    pub command: MuxCommand,
}

/// Operations supported by the network mux control plane.
///
/// Variants create, attach to, list, stop, or internally serve named mux
/// sessions.
///
/// # Examples
///
/// ```
/// use codetether_agent::cli::command::mux_args::MuxCommand;
///
/// let command = MuxCommand::Attach { target: "work".into() };
/// assert!(matches!(command, MuxCommand::Attach { target } if target == "work"));
/// ```
#[derive(Debug, Subcommand)]
pub enum MuxCommand {
    /// Create a persistent named mux session.
    New {
        /// Unique mux session name.
        #[arg(short = 's', long)]
        session: String,
        /// Initial window workspace.
        #[arg(short = 'c', long)]
        directory: Option<PathBuf>,
        /// Leave the server detached instead of opening its client.
        #[arg(short = 'd', long)]
        detached: bool,
    },
    /// Attach an interactive network client to a named session.
    Attach {
        /// Mux session to attach.
        #[arg(short = 't', long)]
        target: String,
    },
    /// List persistent mux sessions.
    #[command(alias = "ls")]
    List {
        /// Emit machine-readable JSON.
        #[arg(long)]
        json: bool,
    },
    /// Stop a persistent mux session.
    Kill {
        /// Mux session to stop.
        #[arg(short = 't', long)]
        target: String,
    },
    /// Internal network-server entry point.
    #[command(hide = true)]
    Serve {
        /// Mux session name.
        #[arg(long)]
        session: String,
        /// Initial window workspace.
        #[arg(long)]
        directory: PathBuf,
        /// Loopback endpoint to bind.
        #[arg(long, default_value = "127.0.0.1:0")]
        bind: SocketAddr,
    },
}

#[cfg(test)]
#[path = "mux_args_tests.rs"]
mod tests;
