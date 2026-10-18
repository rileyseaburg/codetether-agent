//! Activity tracking, retention and final presentation callbacks.
use super::Stages;
use std::time::Instant;
/// Frontend state surrounding the ordered event stages.
/// # Implementors
/// Implement with the frontend's activity clock, auto-follow and retention state.
pub trait View: Stages {
    /// Time of the most recent session event.
    fn last_event_at(&mut self) -> &mut Option<Instant>;
    /// Whether chat should follow new output.
    fn auto_follow(&self) -> bool;
    /// Move chat to its newest content.
    fn scroll_to_bottom(&mut self);
    /// Trim retained presentation state after stage processing.
    fn trim(&mut self);
    /// Handle an event left over after all stages.
    fn context(&mut self, event: Self::Event);
}
