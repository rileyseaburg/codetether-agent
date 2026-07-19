//! Mux lifecycle subcommands.

use std::net::SocketAddr;
use std::path::PathBuf;

use clap::Subcommand;

/// Operations supported by the network mux control plane.
///
/// # Examples
///
/// ```
/// use codetether_agent::cli::command::mux_args::MuxCommand;
///
/// let command = MuxCommand::Kill {
///     target: "work".into(),
/// };
/// assert!(matches!(command, MuxCommand::Kill { target } if target == "work"));
/// ```
#[derive(Debug, Subcommand)]
pub enum MuxCommand {
    /// Create a persistent named mux session.
    New {
        /// Unique mux session name.
        #[arg(value_name = "SESSION")]
        session: String,
        /// Initial window workspace supplied positionally.
        #[arg(value_name = "DIRECTORY")]
        directory: Option<PathBuf>,
        /// Leave the server detached instead of opening its client.
        #[arg(short = 'd', long)]
        detached: bool,
    },
    /// Attach an interactive network client to a named session.
    Attach {
        /// Mux session to attach.
        #[arg(value_name = "SESSION")]
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
        /// Mux session name.
        #[arg(value_name = "TARGET")]
        target: String,
    },
    /// Stop every persistent mux session.
    #[command(name = "kill-all", alias = "kill-server")]
    KillAll,
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
