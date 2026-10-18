//! Native transcript updates after shared missing-tool retry decisions.
use super::super::super::loop_constants as limits;
use super::super::Runner;
use crate::provider::CompletionResponse;
pub(super) fn native_call(
    runner: &mut Runner<'_>,
    response: &CompletionResponse,
    text: &str,
    calls: bool,
) -> anyhow::Result<bool> {
    if runner.progress.native_retry(
        &runner.model.provider_name,
        &runner.model.model_id,
        &runner.model.tools,
        text,
        calls,
    ) {
        runner.session.add_message(response.message.clone());
        super::nudge::add(runner, limits::NATIVE_TOOL_PROMISE_NUDGE);
        return Ok(true);
    }
    super::build_exhaustion::ensure_calls(runner)
}
