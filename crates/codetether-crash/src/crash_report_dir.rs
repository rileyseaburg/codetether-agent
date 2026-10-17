//! Crash reporting crash report dir.
use super::*;
pub(super) fn crash_report_dir() -> PathBuf {
    codetether_data_dir().join("crash-reports")
}
