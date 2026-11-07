//! Task-session preparation policies independent of application sessions.
//!
//! [normalize_agent] selects supported agent modes. [prepare_git] configures
//! the selected checkout using task metadata and optional provenance.
mod agent;
mod provenance_hook;
mod repository;
#[cfg(test)]
mod repository_tests;
pub use agent::normalize_agent;
pub use repository::prepare_git;

/// Claimed-task session loading, provenance, metadata and Git setup.
pub mod session;
