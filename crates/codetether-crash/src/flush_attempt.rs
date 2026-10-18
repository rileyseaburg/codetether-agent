//! Counters and transport failure recorded during one flush.
#[derive(Default)]
pub(super) struct FlushAttempt {
    pub(super) sent: usize,
    pub(super) failed: usize,
    pub(super) status_failures: usize,
    pub(super) transport_error: Option<anyhow::Error>,
}
