//! Policy-gated model, quality, verification and commit effects.
use codetether_ralph_core::UserStory;
use std::path::Path;
/// Native effects invoked by sequential orchestration.
/// # Implementors
/// Use the existing model loop and command-policy-aware verification services.
#[async_trait::async_trait]
pub trait Execution: Send + Sync {
    /// Execute the story model loop.
    /// # Errors
    /// Returns model or tool execution failures.
    async fn call_llm(&self, story_id: &str, prompt: &str) -> anyhow::Result<String>;
    /// Run configured quality gates and deliver their events.
    /// # Errors
    /// Returns quality-runner failures.
    async fn run_quality_gates_with_events(&self, story_id: &str) -> anyhow::Result<bool>;
    /// Verify story completion even when quality checks are disabled.
    /// # Errors
    /// Returns failed verification or runner errors.
    async fn verify(&self, root: &Path, story: &UserStory) -> anyhow::Result<()>;
    /// Commit a verified story when automatic commits are enabled.
    /// # Errors
    /// Returns the native Git commit error.
    fn commit_story(&self, story: &UserStory) -> anyhow::Result<()>;
}
