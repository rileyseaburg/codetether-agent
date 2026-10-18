//! Guard ordering and early-return semantics for one completion.
use super::Host;
use crate::StepFlow;
use codetether_provider_core::CompletionResponse;
/// Apply response guards and execute any resulting tool calls.
/// # Arguments
/// * `host` — Native guard and effect bindings.
/// * `step` — Current provider step.
/// * `response` — Normalized provider completion.
/// # Returns
/// Whether to continue, continue a goal, or finish the turn.
/// # Errors
/// Propagates control, validation and tool execution errors.
/// # Examples
/// ```no_run
/// # async fn run<H: codetether_prompt_loop::driver::Host>(host: &mut H, response: codetether_provider_core::CompletionResponse) -> anyhow::Result<()> {
/// let _flow = codetether_prompt_loop::driver::handle(host, 1, response).await?;
/// # Ok(()) }
/// ```
pub async fn handle(
    host: &mut impl Host,
    step: usize,
    mut response: CompletionResponse,
) -> anyhow::Result<StepFlow> {
    host.ensure_running().await?;
    let (mut calls, truncated) =
        codetether_tool_call_parser::parse_tool_calls(&response.message.content);
    let text =
        codetether_context_policy::transcript::extract_text_content(&response.message.content);
    if host.tool_first(&text, !calls.is_empty())? {
        return Ok(StepFlow::Continue);
    }
    super::salvage::apply(host, &text, &truncated, &mut response, &mut calls);
    if host.native_call(&response, &text, !calls.is_empty())? {
        return Ok(StepFlow::Continue);
    }
    host.emit(step, &response).await;
    if host.thinking_only(&response) {
        return Ok(StepFlow::Continue);
    }
    if calls.is_empty() && truncated.is_empty() {
        return host.terminal(response).await;
    }
    if host.repair(&response, &calls, &truncated).await {
        return Ok(StepFlow::Continue);
    }
    host.track_writes(step, &calls);
    host.add_message(response.message);
    host.execute(step, calls).await?;
    Ok(StepFlow::Continue)
}
