//! Story context supplied by the native runtime.
use codetether_ralph_core::UserStory;
/// Prompt construction and response learning extraction.
/// # Implementors
/// Bind the existing progress and agent-bus prompt helpers.
pub trait Prompts: Send + Sync {
    /// Drain accumulated bus learnings before constructing the prompt.
    fn bus_collect_learnings(&self) -> Vec<String>;
    /// Build the underlying story prompt.
    fn build_prompt(&self, story: &UserStory) -> String;
    /// Extract learnings from the model response.
    fn extract_learnings(&self, response: &str) -> Vec<String>;
}
