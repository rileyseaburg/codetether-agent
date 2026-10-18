//! Browser-control subcommands.
use crate::OfflineCommand;
use clap::Subcommand;
use std::path::PathBuf;

/// Supported browser-control operations.
///
/// # Examples
/// ```
/// let command = codetether_browser_cli::BrowserCtlCommand::Health;
/// ```
#[derive(Subcommand, Debug)]
pub enum BrowserCtlCommand {
    /// Attach to an existing DevTools endpoint or launch a managed browser
    Start {
        #[arg(long)]
        executable_path: Option<String>,
        #[arg(long)]
        user_data_dir: Option<PathBuf>,
    },
    /// Stop the managed browser session for this process
    Stop,
    /// Show current browser session health
    Health,
    /// List tabs in the attached browser session
    List,
    /// Open a URL in the active tab
    Open { url: String },
    /// Read URL, title, viewport, and visible page text
    Snapshot,
    /// Evaluate JavaScript in the active tab
    Eval {
        expression: String,
        #[arg(long, default_value_t = 30_000)]
        timeout_ms: u64,
    },
    /// Capture a screenshot to a local path
    Screenshot {
        #[arg(long)]
        path: PathBuf,
        #[arg(long, default_value_t = true)]
        full_page: bool,
    },
    /// TetherScript-backed offline probes: auth-trace, cookie-diff, explain-cors, record, replay
    Offline {
        #[command(subcommand)]
        cmd: OfflineCommand,
    },
}
