//! Application tool-call callback for native worker response sequencing.
use super::{Session, ToolCall, execute_tool_call};
/// Concrete session tool-call handling for the native response driver.
pub(in crate::a2a::worker::session_steps) struct Runner;
impl codetether_native_worker::response::Tools<Session, crate::tool::ToolRegistry> for Runner {
    async fn execute(
        session: &mut Session,
        context: &codetether_a2a_worker_core::session_protocol::context::ResponseContext<
            '_,
            crate::tool::ToolRegistry,
        >,
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
