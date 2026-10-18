//! Delegate the agent's public execution entry point to shared completion.
use crate::{Agent, AgentResponse, Host};
impl<H: Host> Agent<H> {
    /// Execute a prompt with the configured provider and tool budget.
    pub async fn execute(
        &self,
        session: &mut H::Session,
        prompt: &str,
    ) -> anyhow::Result<AgentResponse> {
        codetether_agent_completion::execute(self, session, prompt).await
    }
}
