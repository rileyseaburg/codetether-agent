//! BenchmarkConfig benchmark data.
use super::*;

/// Configuration for a benchmark run
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let config = codetether_benchmark::BenchmarkConfig::default();
/// assert_eq!(config.max_iterations, 10);
/// ```
pub struct BenchmarkConfig {
    /// Directory containing benchmark PRD files
    pub prd_dir: String,

    /// Models to benchmark (format: "provider:model")
    pub models: Vec<String>,

    /// Only run PRDs matching this tier (1, 2, or 3). None = all tiers.
    pub tier: Option<u8>,

    /// Run model×PRD combos in parallel
    pub parallel: bool,

    /// Maximum iterations per story
    pub max_iterations: usize,

    /// Timeout per story in seconds
    pub story_timeout_secs: u64,

    /// Output file path
    pub output: String,

    /// Cost ceiling per benchmark run in USD (prevents runaway spending)
    pub cost_ceiling_usd: Option<f64>,

    /// Optional API URL to submit results to
    pub submit_api_url: Option<String>,

    /// Optional API key for result submission
    pub submit_api_key: Option<String>,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            prd_dir: "benchmarks".to_string(),
            models: Vec::new(),
            tier: None,
            parallel: false,
            max_iterations: 10,
            story_timeout_secs: 300,
            output: "benchmark_results.json".to_string(),
            cost_ceiling_usd: Some(50.0),
            submit_api_url: None,
            submit_api_key: None,
        }
    }
}
