//! Root command-line arguments for the `codetether` binary.
//!
//! [`Cli`] owns options accepted before any subcommand and selects the
//! interactive TUI when no [`Command`] is supplied.

use crate::Command;
use std::marker::PhantomData;
#[path = "root_metadata.rs"]
mod metadata;
use clap::Parser;
pub use metadata::ApplicationMetadata;
use std::path::PathBuf;

/// Top-level CodeTether command-line arguments.
///
/// # Examples
///
/// ```
/// use clap::Parser;
/// use codetether_cli_command::root::{Cli, ApplicationMetadata};
/// #[derive(Debug)]
/// struct App;
/// impl ApplicationMetadata for App {
///     const VERSION: &'static str = "1.0.0";
///     const ABOUT: &'static str = "Example application";
/// }
///
/// let cli = Cli::<App>::parse_from(["codetether", "--yolo"]);
/// assert!(cli.yolo);
/// assert!(cli.command.is_none());
/// ```
#[derive(Parser, Debug)]
#[command(name = "codetether")]
#[command(version = M::VERSION, about = M::ABOUT, long_about = None)]
#[command(after_help = codetether_models_cli::setup::WORKFLOW)]
pub struct Cli<M: ApplicationMetadata> {
    #[arg(skip)]
    application: PhantomData<fn() -> M>,
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
    /// A2A/MCP control-plane token, NOT a Vault token. For Vault: codetether vault login token.
    #[arg(short, long, env = "CODETETHER_TOKEN", hide_env_values = true)]
    pub token: Option<String>,
    /// Worker name.
    #[arg(short, long, env = "CODETETHER_WORKER_NAME")]
    pub name: Option<String>,
    /// Optional command; omission launches the TUI.
    #[command(subcommand)]
    pub command: Option<Command>,
}
