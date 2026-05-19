//! Context compaction telemetry events.

use crate::session::{CompactionFailure, CompactionOutcome, CompactionStart};
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

pub(crate) fn compaction_started(app: &mut App, start: CompactionStart) {
    app.state.status = format!(
        "Compacting context… {} tokens over {} budget",
        start.before_tokens, start.budget
    );
}

pub(crate) fn compaction_completed(app: &mut App, outcome: CompactionOutcome) {
    app.state.context_used = Some(outcome.after_tokens);
    app.state.context_health.note_compaction(&outcome);
    app.state.status = format!(
        "Context compacted: {} → {} tokens",
        outcome.before_tokens, outcome.after_tokens
    );
}

pub(crate) fn compaction_failed(app: &mut App, failure: CompactionFailure) {
    app.state.status = "Context compaction failed".to_string();
    app.state.messages.push(ChatMessage::new(
        MessageType::Error,
        format!(
            "Context compaction failed: {} ({} / {} tokens)",
            failure.reason, failure.after_tokens, failure.budget
        ),
    ));
}
