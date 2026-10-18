//! Native serialized-tool entry point and input-test adapter.
use crate::{agent::Agent, session::Session, tool::ToolResult};
impl Agent {
    pub(super) async fn execute_tool_for_session(
        &self,
        session: &Session,
        call_id: &str,
        name: &str,
        arguments: &str,
    ) -> ToolResult {
        codetether_agent_tools::execute_for_session(self, session, call_id, name, arguments).await
    }
}
#[cfg(test)]
pub(super) fn prepare(
    session: &Session,
    call_id: &str,
    name: &str,
    arguments: &str,
) -> Result<serde_json::Value, String> {
    codetether_agent_tools::prepare::<Agent>(session, call_id, name, arguments)
}
