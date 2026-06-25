//! Processes a completed model response: records usage, emits output, runs any
//! tool calls. Kept separate so the step loop stays within the file budget.

use std::sync::Arc;

use super::session_step_tools::{append_text_output, collect_tool_calls, execute_tool_call};
use crate::provider::CompletionResponse;
use crate::session::Session;

/// Constant per-run inputs needed to process a step response.
pub(super) struct ResponseContext<'a> {
    pub model: &'a str,
    pub tool_registry: &'a crate::tool::ToolRegistry,
    pub auto_approve: super::super::AutoApprove,
    pub workspace_dir: &'a std::path::Path,
    pub output_callback: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
}

/// Record usage, append text output, persist the message, then run tool calls.
/// Returns `true` when there were no tool calls and the loop should stop.
pub(super) async fn process_response(
    ctx: &ResponseContext<'_>,
    session: &mut Session,
    final_output: &mut String,
    response: CompletionResponse,
) -> bool {
    let (pt, ct) = (
        response.usage.prompt_tokens,
        response.usage.completion_tokens,
    );
    crate::telemetry::TOKEN_USAGE.record_model_usage(ctx.model, pt as u64, ct as u64);
    let tool_calls = collect_tool_calls(&response.message.content);
    append_text_output(
        &response.message.content,
        final_output,
        &ctx.output_callback,
    );
    session.add_message(response.message.clone());
    if tool_calls.is_empty() {
        return true;
    }
    for tool_call in tool_calls {
        execute_tool_call(
            session,
            ctx.tool_registry,
            ctx.auto_approve,
            ctx.workspace_dir,
            ctx.model,
            &ctx.output_callback,
            tool_call,
        )
        .await;
    }
    false
}
