//! Context/RLM health snapshot stored by the TUI.

use crate::session::{CompactionOutcome, ContextTruncation, RlmCompletion, RlmProgressEvent};

#[derive(Debug, Clone, Default)]
pub struct ContextHealthState {
    pub last_compaction: Option<String>,
    pub last_rlm: Option<String>,
    pub last_truncation: Option<String>,
}

impl ContextHealthState {
    pub fn note_compaction(&mut self, event: &CompactionOutcome) {
        self.last_compaction = Some(format!(
            "{}: {} → {} tokens ({:.0}% reduction, kept {} msgs)",
            event.strategy.as_str(),
            event.before_tokens,
            event.after_tokens,
            event.reduction() * 100.0,
            event.kept_messages
        ));
    }

    pub fn note_rlm_progress(&mut self, event: &RlmProgressEvent) {
        self.last_rlm = Some(format!(
            "running {}/{} ({:.0}%): {}",
            event.iteration,
            event.max_iterations,
            event.fraction() * 100.0,
            event.status
        ));
    }

    pub fn note_rlm_complete(&mut self, event: &RlmCompletion) {
        self.last_rlm = Some(format!(
            "{:?}: {} → {} tokens, {} iter, {} ms",
            event.outcome,
            event.input_tokens,
            event.output_tokens,
            event.iterations,
            event.elapsed_ms
        ));
    }

    pub fn note_truncation(&mut self, event: &ContextTruncation) {
        self.last_truncation = Some(format!(
            "dropped {} tokens, kept {} msgs",
            event.dropped_tokens, event.kept_messages
        ));
    }
}
