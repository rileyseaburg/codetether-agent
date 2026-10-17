//! BenchmarkSubmission benchmark data.
use super::*;

/// Submission payload for the benchmark API
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// use codetether_benchmark::*;
/// let result = BenchmarkSubmission { model: String::new(), agent: String::new(), result: String::new(), };
/// let _ = result;
/// ```
pub struct BenchmarkSubmission {
    pub model: String,
    pub agent: String,
    pub result: String,
}
