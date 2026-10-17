//! BenchmarkSummary benchmark data.
use super::*;

/// Summary across all models
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// use codetether_benchmark::*;
/// let result = BenchmarkSummary { best_pass_rate_model: String::new(), fastest_model: String::new(), cheapest_model: String::new(), best_overall_model: String::new(), rankings: vec![], };
/// let _ = result;
/// ```
pub struct BenchmarkSummary {
    /// Best model by pass rate
    pub best_pass_rate_model: String,

    /// Best model by speed
    pub fastest_model: String,

    /// Best model by cost efficiency
    pub cheapest_model: String,

    /// Best overall (weighted score)
    pub best_overall_model: String,

    /// Model rankings
    pub rankings: Vec<ModelRanking>,
}
