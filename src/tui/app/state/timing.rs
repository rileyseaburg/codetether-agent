//! Request timing / latency tracking methods.
//!
//! First-token and last-token timestamps per request; completed timings
//! are promoted to `last_request_*` fields for the UI.

impl super::AppState {
    pub fn begin_request_timing(&mut self) {
        let now = self.request_timing.begin_request_timing();
        // Synchronize watchdog activity with the exact request start instant.
        self.main_last_event_at = Some(now);
    }

    pub fn current_request_elapsed_ms(&self) -> Option<u64> {
        self.request_timing.current_request_elapsed_ms()
    }

    pub fn note_text_token(&mut self) {
        self.request_timing.note_text_token();
    }

    pub fn complete_request_timing(&mut self) {
        self.request_timing.complete_request_timing();
    }

    pub fn clear_request_timing(&mut self) {
        self.request_timing.clear_request_timing();
    }

    /// Finalize the current turn and record e2e latency stats.
    ///
    /// Called when `SessionEvent::Done` fires — this is the full
    /// agentic loop (Enter → model calls → tool calls → done).
    pub fn complete_turn_timing(&mut self) {
        self.request_timing.complete_turn_timing();
    }
}
