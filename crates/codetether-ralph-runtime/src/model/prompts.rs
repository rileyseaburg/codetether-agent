//! Supply story and application context to shared prompt helpers.
use super::*;
impl<H: Host> RalphLoop<H> {
    /// Build the prompt for a story
    pub(crate) fn build_prompt(&self, story: &UserStory) -> String {
        let progress = self.load_progress().unwrap_or_default();
        codetether_ralph_core::presentation::build_prompt(story, &self.state.prd, progress)
    }

    /// Extract learnings from response
    pub(crate) fn extract_learnings(&self, response: &str) -> Vec<String> {
        codetether_ralph_core::presentation::extract_learnings(response)
    }
}
