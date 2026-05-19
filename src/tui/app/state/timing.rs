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

    /// Finalize the current turn and record e2e latency stats.
    ///
    /// Called when `SessionEvent::Done` fires — this is the full
    /// agentic loop (Enter → model calls → tool calls → done).
    pub fn complete_turn_timing(&mut self) {
        let e2e_ms = self.current_request_elapsed_ms();
        let ttft_ms = self.current_request_first_token_ms;
        if let Some(e2e) = e2e_ms {
            self.chat_latency.record(e2e, ttft_ms);
        }
        self.complete_request_timing();
    }

    pub fn begin_streaming(&mut self) {
        self.streaming_start = Some(Instant::now());
        self.streaming_chars = 0;
    }

    pub fn record_streaming_chars(&mut self, len: usize) {
        self.streaming_chars = self.streaming_chars.saturating_add(len);
    }

    pub fn streaming_tok_per_sec(&self) -> Option<f64> {
        let start = self.streaming_start?;
        let elapsed = start.elapsed().as_secs_f64();
        if elapsed < 0.1 {
            return None;
        }
        let tokens = self.streaming_chars as f64 / 4.0;
        Some(tokens / elapsed)
    }
}
