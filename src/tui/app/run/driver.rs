//! Bind shared startup sequencing to native UI/session operations.
use crate::{a2a::spawn::SpawnOptions, config::AccessMode};
/// TUI command launcher using the application's existing startup operations.
pub type Cli = codetether_tui_startup::cli::ApplicationHost<super::host::Native>;
/// Start and run the native terminal UI using the shared startup lifecycle.
pub async fn run(
    project: Option<std::path::PathBuf>,
    allow_network: bool,
    a2a_options: Option<SpawnOptions>,
    access_mode: Option<AccessMode>,
    yolo: bool,
    session_id: Option<String>,
) -> anyhow::Result<()> {
    codetether_tui_startup::run::<super::host::Native>(
        project,
        allow_network,
        a2a_options,
        access_mode,
        yolo,
        session_id,
    )
    .await
}
