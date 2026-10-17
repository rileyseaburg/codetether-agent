//! Crash reporting crash upload state path.
use super::*;
pub(super) fn crash_upload_state_path(settings: &CrashReporterSettings) -> PathBuf {
    // Keep it next to install_id under telemetry/ so it's easy to find.
    // Use settings.report_dir's sibling to avoid depending on OS-specific paths.
    // report_dir = <data>/crash-reports
    // state_dir  = <data>/telemetry
    let data_dir = settings
        .report_dir
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(codetether_data_dir);
    data_dir.join("telemetry").join(CRASH_UPLOAD_STATE_FILE)
}
