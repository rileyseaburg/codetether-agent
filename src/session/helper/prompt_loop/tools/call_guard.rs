//! Event setup and pre-execution rejection for one tool call.

use super::{super::Runner, call::Call};

/// Emits a bounded tool-start event when streaming is enabled.
pub(super) async fn publish_start(runner: &Runner<'_>, call: &Call) {
    let Some(tx) = &runner.events else {
        return;
    };
    let args = serde_json::to_string(&call.input).unwrap_or_default();
    super::super::super::tool_event_emit::start(
        tx,
        &call.id,
        &call.name,
        super::super::super::event_payload::bounded_tool_arguments(&args),
    )
    .await;
}

/// Returns the reason an interactive, stubbed, or repeated call is blocked.
pub(super) fn blocked(runner: &mut Runner<'_>, call: &Call) -> Option<String> {
    if super::super::super::runtime::is_interactive_tool(&call.name) {
        return Some("Interactive tool 'question' is disabled in this interface. Ask the user directly in assistant text.".into());
    }
    super::super::super::edit::detect_stub_in_tool_input(&call.name, &call.input)
        .or_else(|| runner.progress.repeat_guard.check(&call.name, &call.input))
}
