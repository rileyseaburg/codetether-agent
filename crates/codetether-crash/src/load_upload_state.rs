//! Crash reporting load upload state.
use super::*;
pub(super) async fn load_upload_state(
    settings: &CrashReporterSettings,
) -> Option<CrashUploadState> {
    let path = crash_upload_state_path(settings);
    let raw = tokio::fs::read_to_string(&path).await.ok()?;
    serde_json::from_str(&raw).ok()
}
