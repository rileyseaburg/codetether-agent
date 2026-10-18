//! Native terminal runtime entry point.
use codetether_a2a_http::peer_startup::SpawnOptions;
use codetether_config::AccessMode;
use std::{future::Future, path::PathBuf};
/// Application callback for the resolved TUI launch settings.
/// # Implementors
/// Forward the settings to the existing terminal driver without reinterpreting them.
pub trait Host {
    /// Start the native TUI with resolved project, policy and peer options.
    /// # Errors
    /// Returns the terminal driver's startup or execution error.
    fn run(
        project: Option<PathBuf>,
        allow_network: bool,
        a2a_options: Option<SpawnOptions>,
        access_mode: Option<AccessMode>,
        yolo: bool,
        session_id: Option<String>,
    ) -> impl Future<Output = anyhow::Result<()>>;
}
