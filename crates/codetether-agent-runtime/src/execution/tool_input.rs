//! Native serialized-tool entry point and input-test adapter.
use crate::{Agent, Host};
use codetether_tool_core::ToolResult;
impl<H: Host> Agent<H> {
    pub(super) async fn execute_tool_for_session(
        &self,
        session: &H::Session,
        call_id: &str,
        name: &str,
        arguments: &str,
    ) -> ToolResult {
        codetether_agent_tools::execute_for_session(self, session, call_id, name, arguments).await
    }
}
