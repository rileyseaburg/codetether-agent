//! Preserve native session construction and mutation methods.
use super::*;
#[async_trait::async_trait]
impl codetether_agent_runtime::SessionWrite for Native {
    async fn new_session() -> anyhow::Result<Session> {
        Session::new().await
    }
    fn add_human(session: &mut Session, message: Message) {
        session.add_human_message(message);
    }
    fn add_message(session: &mut Session, message: Message) {
        session.add_message(message);
    }
    fn record_tool(session: &mut Session, usage: ToolUse) {
        session.tool_uses.push(usage);
    }
}
