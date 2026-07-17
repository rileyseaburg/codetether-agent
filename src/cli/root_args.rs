//! Root command-line arguments for the `codetether` binary.
//!
//! [`Cli`] owns options accepted before any subcommand and selects the
//! interactive TUI when no [`Command`] is supplied.

use super::Command;
use clap::Parser;
use std::path::PathBuf;

#[cfg(test)]
#[path = "root_args_tests.rs"]
mod tests;

/// Top-level CodeTether command-line arguments.
///
/// # Examples
///
/// ```
/// use clap::Parser;
/// use codetether_agent::cli::Cli;
///
/// let cli = Cli::parse_from(["codetether", "--yolo"]);
/// assert!(cli.yolo);
/// assert!(cli.command.is_none());
/// ```
#[derive(Parser, Debug)]
#[command(name = "codetether")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Project directory to operate on.
    #[arg(global = true, last = true)]
    pub project: Option<PathBuf>,
    /// Print logs to stderr.
    #[arg(long, global = true)]
    pub print_logs: bool,
    /// Log level.
    #[arg(long, global = true, value_parser = ["DEBUG", "INFO", "WARN", "ERROR"])]
    pub log_level: Option<String>,
    /// Enable full-auto mode in the default TUI.
    #[arg(long)]
    pub yolo: bool,
    /// A2A server URL.
    #[arg(short, long, env = "CODETETHER_SERVER")]
    pub server: Option<String>,
    /// Authentication token.
    #[arg(short, long, env = "CODETETHER_TOKEN")]
    pub token: Option<String>,
    /// Worker name.
    #[arg(short, long, env = "CODETETHER_WORKER_NAME")]
    pub name: Option<String>,
    /// Optional command; omission launches the TUI.
    #[command(subcommand)]
    pub command: Option<Command>,
}
