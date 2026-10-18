//! Context compaction telemetry events.

use super::View;
use codetether_session_events::{CompactionFailure, CompactionOutcome, CompactionStart};

pub fn compaction_started(view: &mut impl View, start: CompactionStart) {
    view.set_status(format!(
        "Compacting context… {} tokens over {} budget",
        start.before_tokens, start.budget
    ));
}

pub fn compaction_completed(view: &mut impl View, outcome: CompactionOutcome) {
    view.set_used(outcome.after_tokens);
    view.health().note_compaction(&outcome);
    view.set_status(format!(
        "Context compacted: {} → {} tokens",
        outcome.before_tokens, outcome.after_tokens
    ));
}

pub fn compaction_failed(view: &mut impl View, failure: CompactionFailure) {
    view.set_status("Context compaction failed".to_string());
    view.push_error(format!(
        "Context compaction failed: {} ({} / {} tokens)",
        failure.reason, failure.after_tokens, failure.budget
    ));
}
