//! Optional crash-report initialization for full agent commands, not native probes.

use codetether_agent::{config, crash};
use std::io::IsTerminal;

pub(super) async fn initialize(is_tui: bool) {
    let app_config = match config::Config::load().await {
        Ok(config) => config,
        Err(error) => {
            tracing::warn!(%error, "Failed to load config for crash reporter; using defaults");
            config::Config::default()
        }
    };
    let allow_prompt = is_tui && std::io::stdin().is_terminal() && std::io::stdout().is_terminal();
    let app_config = crash::maybe_prompt_for_consent(&app_config, allow_prompt).await;
    crash::initialize(&app_config).await;
}
