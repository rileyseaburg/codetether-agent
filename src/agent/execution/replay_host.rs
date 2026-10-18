//! Native session-policy and tool execution bindings for shared replay.
use crate::{
    agent::{Agent, ToolUse},
    provider::Message,
    session::Session,
    tool::ToolResult,
};
#[async_trait::async_trait]
impl codetether_agent_replay::Host for Agent {
    type Session = Session;
    fn blocked(session: &Session, name: &str, arguments: &str) -> Option<ToolResult> {
        crate::session::helper::runtime::block_serialized_prior_context_for_session(
            session, name, arguments,
        )
    }
    async fn execute_tool_for_session(
        &self,
        session: &Session,
        id: &str,
        name: &str,
        arguments: &str,
    ) -> ToolResult {
        self.execute_tool_for_session(session, id, name, arguments)
            .await
    }
    fn record_tool(session: &mut Session, usage: ToolUse) {
        session.tool_uses.push(usage);
    }
    fn add_message(session: &mut Session, message: Message) {
        session.add_message(message);
    }
}
