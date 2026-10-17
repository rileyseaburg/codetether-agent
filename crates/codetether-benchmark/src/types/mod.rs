//! Benchmark configuration, measurements, rankings, and submission payloads.
//! These serde types retain the application benchmark wire format.
use serde::{Deserialize, Serialize};
mod benchmark_config;
pub use benchmark_config::BenchmarkConfig;
mod benchmark_suite_result;
pub use benchmark_suite_result::BenchmarkSuiteResult;
mod model_benchmark_result;
pub use model_benchmark_result::ModelBenchmarkResult;
mod prd_benchmark_result;
pub use prd_benchmark_result::PrdBenchmarkResult;
mod story_benchmark_result;
pub use story_benchmark_result::StoryBenchmarkResult;
mod quality_check_result;
pub use quality_check_result::QualityCheckResult;
mod aggregate_metrics;
pub use aggregate_metrics::AggregateMetrics;
mod benchmark_summary;
pub use benchmark_summary::BenchmarkSummary;
mod model_ranking;
pub use model_ranking::ModelRanking;
mod benchmark_submission;
pub use benchmark_submission::BenchmarkSubmission;
