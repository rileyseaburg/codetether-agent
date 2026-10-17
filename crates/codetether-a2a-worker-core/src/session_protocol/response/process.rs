//! Record a provider response and sequence its tool calls.
use super::ResponseHost;
use crate::session_protocol::{append_text_output, collect_tool_calls};
use codetether_provider_core::CompletionResponse;
use std::sync::Arc;

/// Record usage, append text output, persist the message, then run tool calls.
/// Returns `true` when there were no tool calls and the loop should stop.
/// # Arguments
/// * `host` - Application effects for this response.
/// * `model` - Model name used for token usage recording.
/// * `output_callback` - Optional observer for nonempty assistant text parts.
/// * `final_output` - Accumulated assistant text.
/// * `response` - Completed provider response.
/// # Returns
/// True when the response has no tool calls, otherwise false.
/// # Errors
/// Returns the host's execution-boundary error before recording the response.
/// # Examples
/// ```rust,no_run
/// # async fn example(host: &mut impl codetether_a2a_worker_core::session_protocol::response::ResponseHost, response: codetether_provider_core::CompletionResponse) -> anyhow::Result<()> {
/// use codetether_a2a_worker_core::session_protocol::response::process_response;
/// let mut output = String::new();
/// let finished = process_response(host, "model", &None, &mut output, response).await?;
/// # Ok(()) }
/// ```
pub async fn process_response<H: ResponseHost>(
    host: &mut H,
    model: &str,
    output_callback: &Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
    final_output: &mut String,
    response: CompletionResponse,
) -> anyhow::Result<bool> {
    host.ensure_running().await?;
    let (pt, ct) = (
        response.usage.prompt_tokens,
        response.usage.completion_tokens,
    );
    host.record_usage(model, pt as u64, ct as u64);
    let tool_calls = collect_tool_calls(&response.message.content);
    append_text_output(&response.message.content, final_output, output_callback);
    host.append_message(response.message.clone());
    if tool_calls.is_empty() {
        return Ok(true);
    }
    for tool_call in tool_calls {
        host.execute_tool_call(tool_call).await;
    }
    Ok(false)
}
