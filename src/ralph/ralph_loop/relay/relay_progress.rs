//! Track relay output convergence and terminal markers.
use super::relay_types::RelayProgress;
use super::*;
impl RelayProgress {
    pub(in crate::ralph::ralph_loop) fn observe(
        &mut self,
        output: String,
        to: &str,
        prompt: &str,
        story_id: &str,
    ) -> bool {
        // Convergence detection
        let normalized = RalphLoop::normalize_for_relay(&output);
        if self.previous_normalized.as_deref() == Some(normalized.as_str()) {
            self.convergence_hits += 1;
        } else {
            self.convergence_hits = 0;
        }
        self.previous_normalized = Some(normalized);

        if self.convergence_hits >= 2 {
            info!(target: "codetether_agent::ralph::ralph_loop::relay_progress", story_id = %story_id, turns = self.turns, "Relay converged");
            self.baton = output;
            return true;
        }

        // Check if story reached terminal state
        if output.contains("STORY_COMPLETE") || output.contains("STORY_BLOCKED") {
            info!(target: "codetether_agent::ralph::ralph_loop::relay_progress", story_id = %story_id, turns = self.turns, "Story reached terminal state via relay");
            self.baton = output;
            return true;
        }

        // Prepare handoff for next agent
        self.baton = RalphLoop::prepare_relay_handoff(&to, &output, prompt);
        false
    }
}
