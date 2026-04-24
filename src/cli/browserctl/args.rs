use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    about = "Control a Chromium browser via the local DevTools Protocol",
    long_about = "Control a local Chromium-family browser through Chrome DevTools Protocol.\n\nUse --ws-url with a ws://.../devtools/browser/... URL or http://127.0.0.1:9222. When omitted, Codetether probes local debug ports before launching a managed browser. Access is local-only and uses whatever browser profile/session that DevTools endpoint exposes."
)]
pub struct BrowserCtlArgs {
    #[arg(long, global = true, env = "CODETETHER_BROWSER_WS_URL")]
    pub ws_url: Option<String>,
    #[arg(long, global = true)]
    pub json: bool,
    #[command(subcommand)]
    pub command: BrowserCtlCommand,
}

#[derive(Subcommand, Debug)]
pub enum BrowserCtlCommand {
    /// Attach to an existing DevTools endpoint or launch a managed browser
    Start {
        #[arg(long, default_value_t = true)]
        headless: bool,
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
}
