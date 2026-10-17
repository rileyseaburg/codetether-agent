use super::AgentOutcome;
use async_trait::async_trait;
/// Host execution of one agent attempt with fixed provider, prompts and tools.
///
/// # Implementors
/// Application adapters invoke the agent loop and preserve its output/exit reason.
#[async_trait]
pub trait AgentAttempt: Sync {
    /// Execute one fresh attempt.
    ///
    /// # Errors
    /// Returns the agent-loop error; the retry policy decides whether to retry it.
    async fn execute(&self) -> AgentOutcome;
}
