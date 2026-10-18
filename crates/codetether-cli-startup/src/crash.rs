//! Optional crash-report initialization for full agent commands, not native probes.

use codetether_config as config;
use codetether_crash as crash;
use std::io::IsTerminal;

/// Initialize crash capture with the caller's application identity.
/// # Arguments
/// Receives TUI mode, application version and the native memory-guard installer.
/// # Returns
/// Completes after consent handling and optional pending-report upload.
/// # Examples
/// ```no_run
/// async fn startup() {
///     codetether_cli_startup::crash::initialize(false, "example", |_| {}).await;
/// }
/// ```
pub async fn initialize(
    is_tui: bool,
    app_version: &str,
    install_memory_guards: impl FnOnce(std::path::PathBuf),
) {
    let app_config = match config::Config::load().await {
        Ok(config) => config,
        Err(error) => {
            tracing::warn!(%error, "Failed to load config for crash reporter; using defaults");
            config::Config::default()
        }
    };
    let allow_prompt = is_tui && std::io::stdin().is_terminal() && std::io::stdout().is_terminal();
    let app_config = crash::maybe_prompt_for_consent(&app_config, allow_prompt).await;
    crash::initialize(&app_config, app_version, install_memory_guards).await;
}
