//! Native public entry point for the shared completion loop.
use crate::{
    agent::{Agent, AgentResponse},
    session::Session,
};
impl Agent {
    /// Execute a prompt with the configured provider and tool budget.
    pub async fn execute(
        &self,
        session: &mut Session,
        prompt: &str,
    ) -> anyhow::Result<AgentResponse> {
        codetether_agent_completion::execute(self, session, prompt).await
    }
}
