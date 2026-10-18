//! Lend agent and session fields without copying configuration.
use super::*;
impl<H: Host> codetether_agent_completion::State for Agent<H> {
    fn info(&self) -> &AgentInfo {
        &self.info
    }
    fn provider(&self) -> &Arc<dyn Provider> {
        &self.provider
    }
    fn definitions(&self) -> Vec<ToolDefinition> {
        H::definitions(&self.tools)
    }
    fn tool_available(session: &H::Session, name: &str) -> bool {
        H::available(session, name)
    }
    fn system_prompt(&self, session: &H::Session) -> String {
        H::compose(&self.system_prompt, session)
    }
    fn add_human_message(session: &mut H::Session, message: Message) {
        H::add_human(session, message);
    }
    fn tool_uses(session: &H::Session) -> &[ToolUse] {
        H::tool_uses(session)
    }
    fn usage(session: &H::Session) -> &Usage {
        H::usage(session)
    }
}
