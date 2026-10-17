//! ModelRanking benchmark data.
use super::*;

/// Ranking for a single model
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// use codetether_benchmark::*;
/// let result = ModelRanking { model: String::new(), pass_rate_score: 0.0, speed_score: 0.0, cost_score: 0.0, overall_score: 0.0, };
/// let _ = result;
/// ```
pub struct ModelRanking {
    /// Model identifier
    pub model: String,

    /// Pass rate score (0-100)
    pub pass_rate_score: f64,

    /// Speed score (0-100, higher = faster)
    pub speed_score: f64,

    /// Cost score (0-100, higher = cheaper)
    pub cost_score: f64,

    /// Overall weighted score
    pub overall_score: f64,
}
