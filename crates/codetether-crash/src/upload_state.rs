//! Crash reporting upload state.
use super::*;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub(super) struct CrashUploadState {
    /// Number of consecutive flush attempts that failed without sending any reports.
    pub(super) consecutive_failures: u32,
    /// Do not attempt upload again until this time (UTC).
    pub(super) next_attempt_at: Option<DateTime<Utc>>,
    /// Last attempt timestamp.
    pub(super) last_attempt_at: Option<DateTime<Utc>>,
    /// Last error (truncated) for diagnostics.
    pub(super) last_error: Option<String>,
}
