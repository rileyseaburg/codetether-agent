//! Benchmark runner using native providers and Ralph execution.
mod application;
/// Application-bound PRD benchmark runner.
pub type BenchmarkRunner = codetether_benchmark_runner::BenchmarkRunner<application::Native>;
