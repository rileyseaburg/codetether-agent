//! Bind shared TUI CLI settings to the native terminal driver.
use codetether_agent::{a2a::spawn::SpawnOptions, config::AccessMode, tui};
use codetether_cli_options::TuiArgs;
use std::path::PathBuf;
struct Native;
impl codetether_tui_cli::Host for Native {
    async fn run(
        project: Option<PathBuf>,
        allow_network: bool,
        a2a_options: Option<SpawnOptions>,
        access_mode: Option<AccessMode>,
        yolo: bool,
        session_id: Option<String>,
    ) -> anyhow::Result<()> {
        tui::run(
            project,
            allow_network,
            a2a_options,
            access_mode,
            yolo,
            session_id,
        )
        .await
    }
}
pub(super) async fn execute(
    args: Option<TuiArgs>,
    project: Option<PathBuf>,
    yolo: bool,
) -> anyhow::Result<()> {
    codetether_tui_cli::execute::<Native>(args, project, yolo).await
}
