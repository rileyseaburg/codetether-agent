//! Helpers for the in-flight tool fields on [`super::AppState`].

use std::time::Instant;

impl super::AppState {
    /// Mark `name` as the currently-running tool and record when it started.
    pub fn start_pending_tool(&mut self, name: String) {
        self.pending_tool_name = Some(name);
        self.pending_tool_started_at = Some(Instant::now());
    }

    /// Record the tool result fields and clear the in-flight marker.
    pub fn note_tool_completed(&mut self, name: String, duration_ms: u64, success: bool) {
        self.last_tool_name = Some(name);
        self.last_tool_latency_ms = Some(duration_ms);
        self.last_tool_success = Some(success);
        self.pending_tool_name = None;
        self.pending_tool_started_at = None;
    }

    /// Borrowed view of the in-flight tool, if any: `(name, started_at)`.
    pub fn pending_tool_snapshot(&self) -> Option<(&str, Instant)> {
        self.pending_tool_name
            .as_deref()
            .zip(self.pending_tool_started_at)
    }
}
