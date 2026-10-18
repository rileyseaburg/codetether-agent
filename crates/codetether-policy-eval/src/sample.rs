//! Per-policy comparison sample.
/// One Pareto sample for a single derivation policy run.
///
/// Lower is better on [`Self::context_tokens`], [`Self::fault_count`],
/// and [`Self::oracle_gap`]. Higher is better on [`Self::reuse_rate`].
/// [`Self::kept_messages`] is neutral (informational).
/// # Examples
/// ```
/// let result = codetether_policy_eval::PolicyRunResult {
///     policy: "legacy", kept_messages: 4, context_tokens: 100,
///     fault_count: 0, oracle_gap: 0, reuse_rate: 0.5,
/// };
/// assert_eq!(result.context_tokens, 100);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PolicyRunResult {
    /// Policy identifier matching the derivation policy name.
    pub policy: &'static str,
    /// Size of the derived context in messages.
    pub kept_messages: usize,
    /// Estimated token cost of the derived context.
    pub context_tokens: usize,
    /// Explicit faults raised during the run (ClawVM Tier-1).
    pub fault_count: usize,
    /// Online-minus-oracle fault delta from replay demand analysis.
    pub oracle_gap: usize,
    /// Fraction of derived-context entries that were already present
    /// in the previous turn's derived context (Cognitive-Workspace
    /// warm-hit proxy). `0.0 ≤ reuse_rate ≤ 1.0`.
    pub reuse_rate: f64,
}
