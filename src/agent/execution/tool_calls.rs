//! Delegate tool-call batching and replay to its dedicated crate.
use crate::{agent::Agent, provider::ContentPart, session::Session};
impl Agent {
    pub(super) async fn execute_tool_calls(
        &self,
        session: &mut Session,
        tool_calls: Vec<ContentPart>,
    ) {
        codetether_agent_replay::execute(self, session, tool_calls).await;
    }
}
