//! Exact and whitespace-tolerant matching plans.

mod build;
mod candidates;
mod error;
mod normalize;
mod plan;

pub use plan::MatchPlan;
