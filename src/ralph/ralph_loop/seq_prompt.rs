//! Sequential Ralph prompt handling.
use super::*;
impl RalphLoop {
    pub(super) fn sequential_prompt(&self, story: &UserStory) -> String {
        // Collect accumulated bus learnings for prompt injection
        let bus_learnings = self.bus_collect_learnings();

        // Build the prompt (with bus learnings if available)
        let prompt = if bus_learnings.is_empty() {
            self.build_prompt(story)
        } else {
            let mut p = self.build_prompt(story);
            p.push_str("\n## Learnings from Previous Iterations:\n");
            for l in &bus_learnings {
                p.push_str(&format!("- {l}\n"));
            }
            p
        };

        prompt
    }
}
