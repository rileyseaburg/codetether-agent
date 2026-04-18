//! Request timing / latency tracking methods.
//!
//! First-token and last-token timestamps per request; completed timings
//! are promoted to `last_request_*` fields for the UI.

use std::time::Instant;

impl super::AppState {
    pub fn begin_request_timing(&mut self) {
        self.processing_started_at = Some(Instant::now());
        self.current_request_first_token_ms = None;
        self.current_request_last_token_ms = None;
    }

    pub fn current_request_elapsed_ms(&self) -> Option<u64> {
        self.processing_started_at
            .map(|started| started.elapsed().as_millis() as u64)
    }

    pub fn note_text_token(&mut self) {
        let Some(elapsed_ms) = self.current_request_elapsed_ms() else {
            return;
        };
        if self.current_request_first_token_ms.is_none() {
            self.current_request_first_token_ms = Some(elapsed_ms);
        }
        self.current_request_last_token_ms = Some(elapsed_ms);
    }

    pub fn complete_request_timing(&mut self) {
        self.last_request_first_token_ms = self.current_request_first_token_ms;
        self.last_request_last_token_ms = self.current_request_last_token_ms;
        self.clear_request_timing();
    }

    pub fn clear_request_timing(&mut self) {
        self.processing_started_at = None;
        self.current_request_first_token_ms = None;
        self.current_request_last_token_ms = None;
    }
}
