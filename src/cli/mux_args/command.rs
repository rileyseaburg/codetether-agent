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
///     target: Some("work".into()),
///     named_target: None,
/// };
/// assert!(matches!(command, MuxCommand::Kill { target: Some(_), .. }));
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
        /// Mux session name supplied positionally.
        #[arg(value_name = "TARGET", conflicts_with = "named_target")]
        target: Option<String>,
        /// Mux session name supplied with the tmux-style option.
        #[arg(short = 't', long = "target", value_name = "TARGET")]
        named_target: Option<String>,
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
