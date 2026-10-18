//! Bind story context and learning extraction to existing helpers.
use super::*;
impl<H: Host> codetether_ralph_sequential::Prompts for RalphLoop<H> {
    fn bus_collect_learnings(&self) -> Vec<String> {
        self.bus_collect_learnings()
    }
    fn build_prompt(&self, story: &UserStory) -> String {
        self.build_prompt(story)
    }
    fn extract_learnings(&self, response: &str) -> Vec<String> {
        self.extract_learnings(response)
    }
}
