//! Crash reporting save upload state.
use super::*;
pub(super) async fn save_upload_state(
    state: CrashUploadState,
    settings: &CrashReporterSettings,
) -> Result<()> {
    let path = crash_upload_state_path(settings);
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .with_context(|| format!("create crash telemetry dir {}", parent.display()))?;
    }
    let raw = serde_json::to_string_pretty(&state).context("serialize crash upload state")?;
    tokio::fs::write(&path, raw)
        .await
        .with_context(|| format!("write crash upload state {}", path.display()))?;
    Ok(())
}
