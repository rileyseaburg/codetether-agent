//! AggregateMetrics benchmark data.
use super::*;

/// Aggregate metrics across multiple PRDs
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// use codetether_benchmark::*;
/// let result = AggregateMetrics { prds_attempted: 0, prds_fully_passed: 0, overall_pass_rate: 0.0, total_stories: 0, total_stories_passed: 0, avg_seconds_per_story: 0.0, avg_tokens_per_story: 0.0, total_cost_usd: 0.0, avg_cost_per_story: 0.0, total_duration_seconds: 0.0, stories_per_hour: 0.0, };
/// let _ = result;
/// ```
pub struct AggregateMetrics {
    /// Total PRDs attempted
    pub prds_attempted: usize,

    /// PRDs with 100% pass rate
    pub prds_fully_passed: usize,

    /// Overall story pass rate
    pub overall_pass_rate: f64,

    /// Total stories across all PRDs
    pub total_stories: usize,

    /// Total stories passed
    pub total_stories_passed: usize,

    /// Average time per story (seconds)
    pub avg_seconds_per_story: f64,

    /// Average tokens per story
    pub avg_tokens_per_story: f64,

    /// Total cost
    pub total_cost_usd: f64,

    /// Average cost per story
    pub avg_cost_per_story: f64,

    /// Total duration
    pub total_duration_seconds: f64,

    /// Stories per hour throughput
    pub stories_per_hour: f64,
}
