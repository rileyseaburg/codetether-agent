//! Application context and adapter for the shared worker response driver.
use crate::provider::CompletionResponse;
use crate::session::Session;
#[path = "session_response_host.rs"]
mod host;
pub(super) type ResponseContext<'a> =
    codetether_a2a_worker_core::session_protocol::context::ResponseContext<
        'a,
        crate::tool::ToolRegistry,
    >;
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
