//! Benchmark types - result structures and configuration

use serde::{Deserialize, Serialize};

/// Configuration for a benchmark run
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Complete results from a benchmark suite run
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Results for a single model across all PRDs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelBenchmarkResult {
    /// Model identifier (e.g., "anthropic:claude-sonnet-4-20250514")
    pub model: String,

    /// Per-PRD results for this model
    pub prd_results: Vec<PrdBenchmarkResult>,

    /// Aggregate metrics for this model
    pub aggregate: AggregateMetrics,
}

/// Results for a single PRD run
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Result for a single story within a PRD benchmark
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Quality check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityCheckResult {
    /// Check name (typecheck, test, lint, build)
    pub name: String,

    /// Whether it passed
    pub passed: bool,

    /// Output/error message if failed
    pub output: Option<String>,
}

/// Aggregate metrics across multiple PRDs
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Summary across all models
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Ranking for a single model
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// Submission payload for the benchmark API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkSubmission {
    pub model: String,
    pub agent: String,
    pub result: String,
}

/// Detect tier from PRD filename (e.g., "t1-rest-api.json" -> 1)
pub fn detect_tier(filename: &str) -> u8 {
    if filename.starts_with("t1-") {
        1
    } else if filename.starts_with("t2-") {
        2
    } else if filename.starts_with("t3-") {
        3
    } else {
        2 // default to medium
    }
}
