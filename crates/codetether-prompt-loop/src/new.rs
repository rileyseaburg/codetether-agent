//! Initial values for a new prompt-loop turn.
use crate::LoopState;
use uuid::Uuid;

impl LoopState {
    /// Creates zeroed progress state for the requested step budget.
    pub fn new(max_steps: Option<usize>) -> Self {
        Self {
            output: String::new(),
            max_steps,
            validation_retries: 0,
            decline_guard: Default::default(),
            codesearch_misses: 0,
            build_retries: 0,
            native_retries: 0,
            turn_id: Uuid::new_v4().to_string(),
            goal_accounted_at: std::time::Instant::now(),
            goal_failure_signature: None,
            goal_failure_repeats: 0,
        }
    }
}
