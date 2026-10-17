//! ModelBenchmarkResult benchmark data.
use super::*;

/// Results for a single model across all PRDs
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// use codetether_benchmark::*;
/// # fn example(aggregate: AggregateMetrics) {
/// let result = ModelBenchmarkResult { model: String::new(), prd_results: vec![], aggregate: aggregate, };
/// let _ = result;
/// # }
/// ```
pub struct ModelBenchmarkResult {
    /// Model identifier (e.g., "anthropic:claude-sonnet-4-20250514")
    pub model: String,

    /// Per-PRD results for this model
    pub prd_results: Vec<PrdBenchmarkResult>,

    /// Aggregate metrics for this model
    pub aggregate: AggregateMetrics,
}
