//! Counters, guards and output retained during one prompt turn.

/// Progress counters retained across prompt-loop steps.
/// # Examples
/// ```
/// let state = codetether_prompt_loop::LoopState::new(Some(5));
/// assert_eq!(state.max_steps, Some(5));
/// assert!(state.output.is_empty());
/// ```
pub struct LoopState {
    /// Assistant text accumulated for the final result.
    pub output: String,
    /// Optional maximum provider/tool iterations allowed for the turn.
    pub max_steps: Option<usize>,
    /// Number of failed post-edit validation attempts.
    pub validation_retries: u8,
    /// Guard against repeated edit-family invocations.
    pub repeat_guard: codetether_repeat_guard::RepeatGuard,
    /// Exact tool invocations declined by the user during this turn.
    pub decline_guard: codetether_runtime_policy::decline::DeclineGuard,
    /// Consecutive code-search calls returning no matches.
    pub codesearch_misses: u32,
    /// Build-mode retries waiting for an execution tool call.
    pub build_retries: u8,
    /// Retries after prose promised but omitted a native tool call.
    pub native_retries: u8,
    /// Steps elapsed since the last file-writing tool call.
    pub steps_since_write: u32,
    /// Correlation identifier used for agent-bus messages.
    pub turn_id: String,
    /// Baseline for durable goal elapsed-time accounting.
    pub goal_accounted_at: std::time::Instant,
    /// Diagnostic signature of the latest exhausted goal failure.
    pub goal_failure_signature: Option<String>,
    /// Consecutive goal turns stopped by that same failure.
    pub goal_failure_repeats: u8,
}
