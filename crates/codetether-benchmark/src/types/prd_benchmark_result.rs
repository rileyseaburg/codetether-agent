//! PrdBenchmarkResult benchmark data.
use super::*;

/// Results for a single PRD run
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// use codetether_benchmark::*;
/// let result = PrdBenchmarkResult { prd_id: String::new(), prd_tier: 0, prd_feature: String::new(), stories_total: 0, stories_passed: 0, pass_rate: 0.0, duration_seconds: 0.0, tokens_used: 0, cost_usd: 0.0, quality_checks: vec![], per_story: vec![], };
/// let _ = result;
/// ```
pub struct PrdBenchmarkResult {
    /// PRD identifier (derived from filename)
    pub prd_id: String,

    /// Tier classification
    pub prd_tier: u8,

    /// PRD title/feature name
    pub prd_feature: String,

    /// Total stories in the PRD
    pub stories_total: usize,

    /// Stories that passed all quality gates
    pub stories_passed: usize,

    /// Pass rate (0.0 to 1.0)
    pub pass_rate: f64,

    /// Total duration in seconds
    pub duration_seconds: f64,

    /// Total LLM tokens consumed
    pub tokens_used: u64,

    /// Estimated cost in USD
    pub cost_usd: f64,

    /// Quality check results
    pub quality_checks: Vec<QualityCheckResult>,

    /// Per-story results
    pub per_story: Vec<StoryBenchmarkResult>,
}
