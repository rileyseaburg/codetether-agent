//! # Recursive Language Model (RLM)
//!
//! Re-exports core types from [`codetether_rlm`] crate.
//! The `router` module lives in the crate; this module provides the
//! `RlmRouter` adapter and host-side REPL/tools plumbing.
//!
//! Based on "Recursive Language Models" (Zhang et al. 2025)

// Core types from the workspace crate
pub use codetether_rlm::*;

// Local driver modules and host adapter
pub mod executor_bridge;
pub mod repl;
pub mod router;
pub mod tools;

// Re-export RLM entry points at the rlm:: level for existing call sites.
pub use executor_bridge::RlmExecutor;
pub use router::RlmRouter;
