//! Crash reporting envelope.
use super::*;
#[derive(Debug, Clone, Serialize)]
pub(super) struct CrashEnvelope<'a> {
    pub(super) schema: &'static str,
    pub(super) source: &'static str,
    pub(super) sent_at: DateTime<Utc>,
    pub(super) install_id: &'a str,
    pub(super) report: &'a CrashReport,
}
