//! Native field projection for shared tool-result archival.
use super::{super::Runner, call::Call, outcome::Outcome};
pub(super) fn write(runner: &Runner<'_>, call: &Call, outcome: &Outcome) {
    codetether_event_stream::tool_archive::write(codetether_event_stream::tool_archive::Record {
        session_id: &runner.session.id,
        message_count: runner.session.messages.len() as u64,
        tool_name: &call.name,
        success: outcome.success,
        duration_ms: outcome.duration_ms,
        rendered: &outcome.rendered,
    });
}
