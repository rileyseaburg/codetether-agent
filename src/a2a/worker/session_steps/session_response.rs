//! Application context and adapter for the shared worker response driver.
use crate::provider::CompletionResponse;
use crate::session::Session;
use std::sync::Arc;
#[path = "session_response_host.rs"]
mod host;

/// Constant per-run inputs needed to process a step response.
pub(super) struct ResponseContext<'a> {
    pub model: &'a str,
    pub tool_registry: &'a crate::tool::ToolRegistry,
    pub auto_approve: super::super::AutoApprove,
    pub workspace_dir: &'a std::path::Path,
    pub output_callback: Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
}
struct ApplicationHost<'a, 'context> {
    context: &'a ResponseContext<'context>,
    session: &'a mut Session,
}

/// Record usage, append text output, persist the message, then run tool calls.
/// Returns `true` when there were no tool calls and the loop should stop.
pub(super) async fn process_response(
    ctx: &ResponseContext<'_>,
    session: &mut Session,
    final_output: &mut String,
    response: CompletionResponse,
) -> anyhow::Result<bool> {
    codetether_a2a_worker_core::session_protocol::response::process_response(
        &mut ApplicationHost {
            context: ctx,
            session,
        },
        ctx.model,
        &ctx.output_callback,
        final_output,
        response,
    )
    .await
}
