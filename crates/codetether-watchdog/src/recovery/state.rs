use crate::WatchdogNotification;
use codetether_chat_model::ChatMessage;
use std::time::Instant;
/// Disjoint UI state needed to cancel or retry a stalled request.
/// # Examples
/// Fields borrow the existing UI state; no session data is cloned by this view.
pub struct State<'a> {
    /// Pending stall notification, cleared when retry begins or gives up.
    pub notification: &'a mut Option<WatchdogNotification>,
    /// Current UI processing flag.
    pub processing: &'a mut bool,
    /// Number of automatic restarts already attempted.
    pub restarts: &'a mut u32,
    /// Exact prompt belonging to the stalled turn.
    pub inflight: &'a mut Option<String>,
    /// Last event time used to rearm detection.
    pub last_event: &'a mut Option<Instant>,
    /// Visible recovery status.
    pub status: &'a mut String,
    /// Visible recovery messages.
    pub messages: &'a mut Vec<ChatMessage>,
}
/// Native UI effects interleaved with recovery state changes.
/// # Implementors
/// Applications expose their existing fields and timing/streaming methods.
pub trait Ui {
    /// Borrow the existing recovery fields.
    fn state(&mut self) -> State<'_>;
    /// Clear streaming text after cancellation or permanent failure.
    fn clear_streaming(&mut self);
    /// Clear current request timing.
    fn clear_timing(&mut self);
    /// Start a new retry's request timing.
    fn begin_timing(&mut self);
    /// Scroll to the latest recovery message.
    fn scroll(&mut self);
}
