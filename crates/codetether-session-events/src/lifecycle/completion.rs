//! State cleanup performed after a completed turn.
/// Native completion operations in the order selected by the shared lifecycle.
/// # Implementors
/// Frontends preserve their inbox, timing and watchdog field semantics.
pub trait Completion {
    /// Clear the live text preview.
    fn clear_streaming_text(&mut self);
    /// Promote current timing into the last completed request.
    fn complete_turn_timing(&mut self);
    /// Clear the streaming start instant.
    fn clear_streaming_start(&mut self);
    /// Close the play-break view.
    fn clear_interlude(&mut self);
    /// Clear a handled watchdog notification while retaining its restart count.
    fn clear_watchdog_notification(&mut self);
    /// Clear the completed prompt tracking slot.
    fn clear_inflight_prompt(&mut self);
}
