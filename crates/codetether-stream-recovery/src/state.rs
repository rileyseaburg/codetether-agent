//! Pending prompt and retry count for same-model stream recovery.
/// Recovery state for one interactive request stream.
///
/// # Examples
/// ```
/// let state = codetether_stream_recovery::StreamRecoveryState::default();
/// assert_eq!(state.stream_reconnect_count, 0);
/// assert!(state.pending_stream_reconnect.is_none());
/// ```
#[derive(Debug, Clone, Default)]
pub struct StreamRecoveryState {
    /// Number of same-model retry attempts scheduled.
    pub stream_reconnect_count: u32,
    /// Prompt awaiting resubmission by the host.
    pub pending_stream_reconnect: Option<String>,
}
