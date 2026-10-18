//! Lend agent and session fields without copying configuration.
use super::*;
impl codetether_agent_completion::State for Agent {
    fn info(&self) -> &AgentInfo {
        &self.info
    }
    fn provider(&self) -> &Arc<dyn Provider> {
        &self.provider
    }
    fn definitions(&self) -> Vec<ToolDefinition> {
        self.tools.definitions()
    }
    fn tool_available(session: &Session, name: &str) -> bool {
        crate::session::helper::runtime::prior_context_tool_available_for_session(session, name)
    }
    fn system_prompt(&self, session: &Session) -> String {
        super::super::messages::compose_system_prompt(&self.system_prompt, session)
    }
    fn add_human_message(session: &mut Session, message: Message) {
        session.add_human_message(message);
    }
    fn tool_uses(session: &Session) -> &[ToolUse] {
        &session.tool_uses
    }
    fn usage(session: &Session) -> &Usage {
        &session.usage
    }
}
