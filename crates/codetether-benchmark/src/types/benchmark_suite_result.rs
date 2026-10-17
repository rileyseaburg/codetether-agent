//! BenchmarkSuiteResult benchmark data.
use super::*;

/// Complete results from a benchmark suite run
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// use codetether_benchmark::*;
/// # fn example(summary: BenchmarkSummary) {
/// let result = BenchmarkSuiteResult { run_date: String::new(), agent: String::new(), agent_version: String::new(), model_results: vec![], summary: summary, };
/// let _ = result;
/// # }
/// ```
pub struct BenchmarkSuiteResult {
    /// When the benchmark was run
    pub run_date: String,

    /// Agent being benchmarked
    pub agent: String,

    /// Agent version
    pub agent_version: String,

    /// Per-model results
    pub model_results: Vec<ModelBenchmarkResult>,

    /// Summary across all models
    pub summary: BenchmarkSummary,
}
