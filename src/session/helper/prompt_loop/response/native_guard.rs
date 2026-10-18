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
    if runner.inner.progress.native_retry(
        &runner.inner.model.provider_name,
        &runner.inner.model.model_id,
        &runner.inner.model.tools,
        text,
        calls,
    ) {
        runner.inner.session.add_message(response.message.clone());
        super::nudge::add(runner, limits::NATIVE_TOOL_PROMISE_NUDGE);
        return Ok(true);
    }
    super::build_exhaustion::ensure_calls(runner)
}
