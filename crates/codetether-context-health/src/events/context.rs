//! Context telemetry session events.

use super::compaction::{compaction_completed, compaction_failed, compaction_started};
use super::rlm::{rlm_complete, rlm_progress};

use super::View;
use codetether_session_events::{ContextTruncation, SessionEvent, TokenEstimate};

pub fn token_estimate(view: &mut impl View, est: TokenEstimate) {
    view.set_used(est.request_tokens);
    view.set_budget(est.budget);
}

pub fn truncated(view: &mut impl View, t: ContextTruncation) {
    view.health().note_truncation(&t);
    view.set_status(format!(
        "Context truncated: dropped {} tokens, kept {} messages",
        t.dropped_tokens, t.kept_messages
    ));
}

/// Apply context telemetry to frontend health and status fields.
/// # Arguments
/// * `view` — Mutable context-health state and presentation callbacks.
/// * `evt` — Owned session event; unrelated variants are ignored.
/// # Returns
/// Updates only fields associated with the received context event.
/// # Examples
/// ```no_run
/// # fn example<S>(view: &mut impl codetether_context_health::events::View, event: codetether_session_events::SessionEvent<S>) {
/// codetether_context_health::events::handle_event(view, event);
/// # }
/// ```
pub fn handle_event<S>(view: &mut impl View, evt: SessionEvent<S>) {
    match evt {
        SessionEvent::TokenEstimate(est) => token_estimate(view, est),
        SessionEvent::ContextTruncated(t) => truncated(view, t),
        SessionEvent::CompactionStarted(start) => compaction_started(view, start),
        SessionEvent::CompactionCompleted(outcome) => compaction_completed(view, outcome),
        SessionEvent::CompactionFailed(failure) => compaction_failed(view, failure),
        SessionEvent::RlmProgress(progress) => rlm_progress(view, progress),
        SessionEvent::RlmComplete(done) => rlm_complete(view, done),
        _ => {}
    }
}
