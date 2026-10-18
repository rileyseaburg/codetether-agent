use crate::RetryState;
use std::time::Instant;
/// Borrowed UI state updated as a scheduled model retry proceeds.
/// # Examples
/// Native callers supply independent references from their request state.
pub struct State<'a> {
    /// Pending retry and attempt accounting.
    pub retry: &'a mut RetryState,
    /// Whether execution is currently active.
    pub processing: &'a mut bool,
    /// Prompt retained for active-turn recovery.
    pub inflight: &'a mut Option<String>,
    /// Watchdog activity timestamp.
    pub last_event: &'a mut Option<Instant>,
    /// User-visible status text.
    pub status: &'a mut String,
}
