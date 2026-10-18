//! Browser-control argument schema.

use clap::{ArgAction, Parser};

use crate::BrowserCtlCommand;

/// Global browser-control options and selected command.
///
/// # Examples
/// ```
/// use clap::Parser;
/// let args = codetether_browser_cli::BrowserCtlArgs::parse_from(["browserctl", "health"]);
/// assert!(args.headless);
/// ```
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
    #[arg(long, global = true, default_value_t = true, action = ArgAction::Set)]
    pub headless: bool,
    #[command(subcommand)]
    pub command: BrowserCtlCommand,
}
