//! Supply story and application context to shared prompt helpers.
use super::*;
impl RalphLoop {
    /// Build prompt for a story (static version for parallel execution)
    pub(super) fn build_story_prompt(
        story: &UserStory,
        prd_info: &(String, String),
        working_dir: &Path,
    ) -> String {
        codetether_ralph_core::presentation::build_story_prompt(story, prd_info, working_dir)
    }

    /// Build the prompt for a story
    pub(super) fn build_prompt(&self, story: &UserStory) -> String {
        let progress = self.load_progress().unwrap_or_default();
        codetether_ralph_core::presentation::build_prompt(story, &self.state.prd, progress)
    }

    /// Extract learnings from response
    pub(super) fn extract_learnings(&self, response: &str) -> Vec<String> {
        codetether_ralph_core::presentation::extract_learnings(response)
    }
}
