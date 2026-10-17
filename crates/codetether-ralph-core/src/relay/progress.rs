//! Relay convergence, terminal markers and handoff state.
use super::RelayProgress;
use tracing::info;

impl RelayProgress {
    /// Observe one output and return whether relay execution should stop.
    ///
    /// # Examples
    /// ```
    /// use codetether_ralph_core::relay::RelayProgress;
    /// let mut progress = RelayProgress { baton: String::new(), previous_normalized: None, convergence_hits: 0, turns: 1 };
    /// assert!(progress.observe("STORY_COMPLETE".into(), "coder", "task", "story"));
    /// ```
    pub fn observe(&mut self, output: String, to: &str, prompt: &str, story_id: &str) -> bool {
        // Convergence detection
        let normalized = crate::presentation::normalize_for_relay(&output);
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
        self.baton = crate::presentation::prepare_relay_handoff(&to, &output, prompt);
        false
    }
}
