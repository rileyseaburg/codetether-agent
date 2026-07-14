//! Recording of synthetic or pre-execution tool results.

use super::{super::Runner, call::Call};

/// Records a synthetic or rejected tool result in events and history.
pub(super) async fn record(runner: &mut Runner<'_>, call: &Call, content: String, success: bool) {
    if let Some(tx) = &runner.events {
        super::super::super::tool_event_emit::complete(
            tx,
            &call.id,
            &call.name,
            content.clone(),
            success,
            0,
        )
        .await;
    }
    runner
        .session
        .add_message(super::super::super::tool_output::tool_result_with_status(
            call.id.clone(),
            &call.name,
            success,
            content,
        ));
}
