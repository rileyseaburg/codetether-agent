//! Reset per-turn work counters when an active goal continues.
use crate::LoopState;
impl LoopState {
    /// Reset the same counters used by native goal continuation.
    /// Declined requests and goal-failure accounting remain unchanged.
    /// # Examples
    /// ```
    /// let mut state = codetether_prompt_loop::LoopState::new(None);
    /// state.output.push_str("previous response");
    /// state.reset_goal_turn();
    /// assert!(state.output.is_empty());
    /// ```
    pub fn reset_goal_turn(&mut self) {
        self.output.clear();
        self.validation_retries = 0;
        self.codesearch_misses = 0;
        self.build_retries = 0;
        self.native_retries = 0;
        self.turn_id = uuid::Uuid::new_v4().to_string();
    }
}
