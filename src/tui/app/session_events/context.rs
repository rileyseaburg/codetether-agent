//! Context telemetry session events.

#[path = "compaction.rs"]
mod compaction;
#[path = "rlm.rs"]
mod rlm;

pub(super) use compaction::{compaction_completed, compaction_failed, compaction_started};
pub(super) use rlm::{rlm_complete, rlm_progress};

use crate::session::{ContextTruncation, SessionEvent, TokenEstimate};
use crate::tui::app::state::App;

pub(super) fn token_estimate(app: &mut App, est: TokenEstimate) {
    app.state.context_used = Some(est.request_tokens);
    app.state.context_budget = Some(est.budget);
}

pub(super) fn truncated(app: &mut App, t: ContextTruncation) {
    app.state.context_health.note_truncation(&t);
    app.state.status = format!(
        "Context truncated: dropped {} tokens, kept {} messages",
        t.dropped_tokens, t.kept_messages
    );
}

pub(super) fn handle_event(app: &mut App, evt: SessionEvent) {
    match evt {
        SessionEvent::TokenEstimate(est) => token_estimate(app, est),
        SessionEvent::ContextTruncated(t) => truncated(app, t),
        SessionEvent::CompactionStarted(start) => compaction_started(app, start),
        SessionEvent::CompactionCompleted(outcome) => compaction_completed(app, outcome),
        SessionEvent::CompactionFailed(failure) => compaction_failed(app, failure),
        SessionEvent::RlmProgress(progress) => rlm_progress(app, progress),
        SessionEvent::RlmComplete(done) => rlm_complete(app, done),
        _ => {}
    }
}
