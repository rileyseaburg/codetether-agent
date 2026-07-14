//! Event and transcript publication for completed tool calls.

use super::{super::Runner, call::Call, outcome::Outcome};

/// Publishes and records a completed executable tool result.
pub(super) async fn complete(runner: &mut Runner<'_>, step: usize, call: &Call, outcome: Outcome) {
    super::bus::response(runner, step, call, &outcome);
    super::archive::write(runner, call, &outcome);
    if let Some(tx) = &runner.events {
        super::super::super::tool_metadata_event::send(
            tx,
            &call.id,
            &call.name,
            outcome.metadata.as_ref(),
        )
        .await;
        super::super::super::tool_event_emit::complete(
            tx,
            &call.id,
            &call.name,
            super::super::super::event_payload::bounded_tool_output(&outcome.rendered),
            outcome.success,
            outcome.duration_ms,
        )
        .await;
    }
    let content = super::rlm::route(runner, call, &outcome.rendered);
    runner
        .session
        .add_message(super::super::super::tool_output::tool_result_with_status(
            call.id.clone(),
            &call.name,
            outcome.success,
            content,
        ));
    super::codesearch::record(runner, outcome.codesearch_miss);
}
