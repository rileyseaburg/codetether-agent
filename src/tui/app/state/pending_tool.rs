//! Helpers for the in-flight tool fields on [`super::AppState`].

use std::time::Instant;

impl super::AppState {
    /// Mark `name` as the currently-running tool and record when it started.
    pub fn start_pending_tool(&mut self, name: String) {
        self.tool_timing.start_pending_tool(name);
    }

    /// Record the tool result fields and clear the in-flight marker.
    pub fn note_tool_completed(&mut self, name: String, duration_ms: u64, success: bool) {
        self.tool_timing
            .note_tool_completed(name, duration_ms, success);
    }

    /// Borrowed view of the in-flight tool, if any: `(name, started_at)`.
    pub fn pending_tool_snapshot(&self) -> Option<(&str, Instant)> {
        self.tool_timing.pending_tool_snapshot()
    }
}
