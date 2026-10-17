//! Host interface for sequential scheduling.
use super::*;
/// Supply state, iteration handling, prompts and model execution.
///
/// # Implementors
/// Runtime adapters retain the live provider, persistence and reporting services.
#[async_trait::async_trait]
pub trait SequentialHost: Send + Sync {
    /// Borrow the live iteration state.
    fn state(&self) -> &RalphState;
    /// Start an iteration and select its next story, or stop.
    fn begin(&mut self) -> Option<UserStory>;
    /// Build the prompt for a selected story.
    fn prompt(&self, story: &UserStory) -> String;
    /// Invoke the model; return its output or execution error.
    async fn call(&self, story_id: &str, prompt: &str) -> Result<String>;
    /// Handle successful output and its verification or persistence errors.
    async fn response(&mut self, story: &UserStory, response: &str) -> Result<()>;
    /// Record a model error and continue according to the existing iteration policy.
    fn error(&mut self, story: &UserStory, error: &anyhow::Error);
}
