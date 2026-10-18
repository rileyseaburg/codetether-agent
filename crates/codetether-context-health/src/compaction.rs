//! Compaction and dropped-context labels.
use crate::ContextHealthState;
use codetether_session_events::{CompactionOutcome, ContextTruncation};
impl ContextHealthState {
    /// Record the completed compaction summary.
    pub fn note_compaction(&mut self, event: &CompactionOutcome) {
        self.last_compaction = Some(format!(
            "{}: {} → {} tokens ({:.0}% reduction, kept {} messages)",
            event.strategy.as_str(),
            event.before_tokens,
            event.after_tokens,
            event.reduction() * 100.0,
            event.kept_messages
        ));
    }

    /// Record the dropped-token and retained-message counts.
    pub fn note_truncation(&mut self, event: &ContextTruncation) {
        self.last_truncation = Some(format!(
            "dropped {} tokens, kept {} messages",
            event.dropped_tokens, event.kept_messages
        ));
    }
}
