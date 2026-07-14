//! Transcript repair for tool calls truncated by a provider.

use super::super::Runner;
use crate::provider::CompletionResponse;
type ToolCall = (String, String, serde_json::Value);

/// Records synthetic failures for truncated tool calls and requests a retry.
pub(super) async fn record(
    runner: &mut Runner<'_>,
    response: &CompletionResponse,
    calls: &[ToolCall],
    truncated: &[(String, String)],
) -> bool {
    if truncated.is_empty() {
        return false;
    }
    if calls.is_empty() {
        runner.session.add_message(response.message.clone());
    }
    for (id, name) in truncated {
        let content = super::super::super::tool_truncation::error_content(name);
        if let Some(tx) = &runner.events {
            super::super::super::tool_event_emit::complete(tx, id, name, content.clone(), false, 0)
                .await;
        }
        runner
            .session
            .add_message(super::super::super::tool_output::tool_result_with_status(
                id.clone(),
                name,
                false,
                content,
            ));
    }
    if calls.is_empty() {
        runner
            .session
            .add_message(super::super::super::tool_truncation::retry_prompt(
                truncated,
            ));
        runner.progress.output.clear();
        true
    } else {
        false
    }
}
