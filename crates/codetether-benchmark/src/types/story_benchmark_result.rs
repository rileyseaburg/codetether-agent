//! StoryBenchmarkResult benchmark data.
use super::*;

/// Result for a single story within a PRD benchmark
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// use codetether_benchmark::*;
/// let result = StoryBenchmarkResult { story_id: String::new(), title: String::new(), passed: false, iterations: 0, duration_seconds: 0.0, tokens_used: 0, files_changed: vec![], };
/// let _ = result;
/// ```
pub struct StoryBenchmarkResult {
    /// Story ID
    pub story_id: String,

    /// Story title
    pub title: String,

    /// Whether the story passed
    pub passed: bool,

    /// Number of iterations needed
    pub iterations: usize,

    /// Duration in seconds
    pub duration_seconds: f64,

    /// Tokens used for this story
    pub tokens_used: u64,

    /// Files changed
    pub files_changed: Vec<String>,
}
