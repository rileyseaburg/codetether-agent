//! Routing of one normalized completion response.

use super::super::{Runner, state::StepFlow};
use crate::provider::CompletionResponse;
use anyhow::Result;

/// Applies response guards and dispatches any parsed tool calls.
///
/// # Errors
///
/// Returns an error from validation or tool execution.
pub(in crate::session::helper::prompt_loop) async fn handle(
    runner: &mut Runner<'_>,
    step: usize,
    mut response: CompletionResponse,
) -> Result<StepFlow> {
    let (mut calls, truncated) =
        super::super::super::tool_call_parse::parse_tool_calls(&response.message.content);
    let text = super::super::super::text::extract_text_content(&&response.message.content);
    if super::build_guard::tool_first(runner, &text, !calls.is_empty())? {
        return Ok(StepFlow::Continue);
    }
    super::super::super::tool_extraction::salvage_prose_tool_call(
        &runner.model.provider_name,
        &runner.model.model_id,
        &text,
        &runner.model.tools,
        &truncated,
        &mut response,
        &mut calls,
    );
    if super::native_guard::native_call(runner, &response, &text, !calls.is_empty())? {
        return Ok(StepFlow::Continue);
    }
    super::output::emit(runner, step, &response).await;
    if calls.is_empty() && truncated.is_empty() {
        return super::terminal::finish_or_retry(runner, response).await;
    }
    if super::truncation::record(runner, &response, &calls, &truncated).await {
        return Ok(StepFlow::Continue);
    }
    super::progress::track_writes(runner, step, &calls);
    runner.session.add_message(response.message);
    super::super::tools::execute(runner, step, calls).await?;
    Ok(StepFlow::Continue)
}
