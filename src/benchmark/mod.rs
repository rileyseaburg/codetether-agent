//! Benchmark module - evaluates CodeTether across models using Ralph PRDs
//!
//! Runs standardized PRD-based benchmarks against multiple LLM models,
//! capturing pass rates, timing, token usage, and cost metrics.

mod runner;
mod types;

pub use runner::*;
pub use types::*;
