//! Native adapter for shared truncated-tool-call repair.
use super::super::Runner;
use crate::provider::CompletionResponse;
#[path = "truncation/host.rs"]
mod host;
type ToolCall = (String, String, serde_json::Value);
pub(super) async fn record(
    runner: &mut Runner<'_>,
    response: &CompletionResponse,
    calls: &[ToolCall],
    truncated: &[(String, String)],
) -> bool {
    codetether_prompt_loop::repair::record(
        &mut host::Native {
            session: &mut *runner.inner.session,
            output: &mut runner.inner.progress.output,
            events: runner.inner.events.as_ref(),
        },
        &response.message,
        calls.is_empty(),
        truncated,
    )
    .await
}
