//! Retry guard for prose that promises a native tool call.

use super::super::super::loop_constants as limits;
use super::super::Runner;
use crate::provider::CompletionResponse;
use anyhow::Result;

/// Retries prose that promises but omits a native tool call.
///
/// # Errors
///
/// Returns an error when build mode exhausts its tool-call retries.
pub(super) fn native_call(
    runner: &mut Runner<'_>,
    response: &CompletionResponse,
    text: &str,
    calls: bool,
) -> Result<bool> {
    let retry = super::super::super::provider::should_retry_missing_native_tool_call(
        &runner.model.provider_name,
        &runner.model.model_id,
        runner.progress.native_retries,
        &runner.model.tools,
        text,
        calls,
        limits::NATIVE_TOOL_PROMISE_RETRY_MAX_RETRIES,
    );
    if retry {
        runner.progress.native_retries += 1;
        runner.session.add_message(response.message.clone());
        super::build_guard::nudge(runner, limits::NATIVE_TOOL_PROMISE_NUDGE);
        return Ok(true);
    }
    if calls {
        runner.progress.build_retries = 0;
        runner.progress.native_retries = 0;
    }
    super::build_exhaustion::ensure_calls(runner)
}
