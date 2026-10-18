//! Connect native tool-call execution to the worker response driver.
use super::*;
use codetether_a2a_worker_core::session_protocol::context::ResponseContext;
/// Native response-driver tool runner using the application's input context.
pub struct Runner;
impl<S: InputContext, H: Application> crate::response::Tools<S, ToolRegistry<H>> for Runner {
    async fn execute(
        session: &mut S,
        context: &ResponseContext<'_, ToolRegistry<H>>,
        tool_call: ToolCall,
    ) {
        execute_tool_call(
            session,
            context.tool_registry,
            context.auto_approve,
            context.workspace_dir,
            context.model,
            &context.output_callback,
            tool_call,
        )
        .await;
    }
}
