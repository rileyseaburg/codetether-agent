/// Estimated token usage for a swarm execution.
#[derive(Debug, Clone, Default)]
///
/// # Examples
/// ```
/// let estimate = codetether_swarm_core::validation::TokenEstimate::default();
/// assert_eq!(estimate.total_tokens, 0);
/// ```
pub struct TokenEstimate {
    /// Estimated prompt tokens.
    pub prompt_tokens: usize,
    /// Estimated completion tokens.
    pub completion_tokens: usize,
    /// Estimated total tokens.
    pub total_tokens: usize,
    /// Whether the estimate exceeds the model context window.
    pub exceeds_limit: bool,
    /// Context window of the selected model.
    pub context_window: usize,
}
