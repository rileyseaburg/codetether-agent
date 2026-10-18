//! One proactive summary range and its requested granularity.
use codetether_context_policy::summary_index::{Granularity, SummaryRange};

/// A single transcript range to summarize at a bounded token budget.
/// # Examples
/// ```
/// let plans = codetether_proactive_planning::planned(80);
/// assert!(plans.iter().all(|plan| plan.target_tokens > 0));
/// ```
#[derive(Clone, Copy)]
pub struct Plan {
    /// Half-open transcript range.
    pub range: SummaryRange,
    /// Summary scale for this range.
    pub granularity: Granularity,
    /// Maximum requested summary tokens.
    pub target_tokens: usize,
}
