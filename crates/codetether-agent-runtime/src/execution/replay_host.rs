//! Native session-policy and tool execution bindings for shared replay.
use crate::{Agent, Host, ToolUse};
use codetether_provider_core::Message;
use codetether_tool_core::ToolResult;
#[async_trait::async_trait]
impl<H: Host> codetether_agent_replay::Host for Agent<H> {
    type Session = H::Session;
    fn blocked(session: &H::Session, name: &str, arguments: &str) -> Option<ToolResult> {
        H::prior_blocked(session, name, arguments)
    }
    async fn execute_tool_for_session(
        &self,
        session: &H::Session,
        id: &str,
        name: &str,
        arguments: &str,
    ) -> ToolResult {
        self.execute_tool_for_session(session, id, name, arguments)
            .await
    }
    fn record_tool(session: &mut H::Session, usage: ToolUse) {
        H::record_tool(session, usage);
    }
    fn add_message(session: &mut H::Session, message: Message) {
        H::add_message(session, message);
    }
}
