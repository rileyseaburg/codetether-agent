//! Consecutive empty code-search retry tracking.
use crate::LoopState;
use codetether_prompt_policy::loop_constants as limits;
impl LoopState {
    /// Update empty-search counters only for build agents.
    pub fn record_codesearch(&mut self, agent: &str, missed: bool) {
        if !codetether_build_policy::is_build_agent(agent) {
            return;
        }
        self.codesearch_misses = if missed {
            self.codesearch_misses + 1
        } else {
            0
        };
    }
    /// Return a corrective nudge once the empty-search threshold is reached.
    pub fn codesearch_nudge(&self, step: usize) -> Option<&'static str> {
        if self.codesearch_misses < limits::MAX_CONSECUTIVE_CODESEARCH_NO_MATCHES {
            return None;
        }
        tracing::warn!(
            step,
            consecutive_codesearch_no_matches = self.codesearch_misses,
            "Detected codesearch no-match thrash"
        );
        Some(limits::CODESEARCH_THRASH_NUDGE)
    }
}
