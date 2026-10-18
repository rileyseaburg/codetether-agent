//! RLM execution progress and completion labels.
use crate::ContextHealthState;
use codetether_session_events::{RlmCompletion, RlmProgressEvent};
impl ContextHealthState {
    /// Record the current RLM progress fraction and status.
    pub fn note_rlm_progress(&mut self, event: &RlmProgressEvent) {
        self.last_rlm = Some(format!(
            "running {}/{} ({:.0}%): {}",
            event.iteration,
            event.max_iterations,
            event.fraction() * 100.0,
            event.status
        ));
    }

    /// Record the RLM outcome, token counts and elapsed time.
    pub fn note_rlm_complete(&mut self, event: &RlmCompletion) {
        self.last_rlm = Some(format!(
            "{:?}: {} → {} tokens, {} iterations, {} ms",
            event.outcome,
            event.input_tokens,
            event.output_tokens,
            event.iterations,
            event.elapsed_ms
        ));
    }
}
