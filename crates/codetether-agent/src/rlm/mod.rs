//! # Recursive Language Model (RLM)
//!
//! Re-exports core types from [`codetether_rlm`] crate.
//! The `router` module lives in the crate; this module provides the
//! `RlmRouter` adapter, public compatibility exports, and host configuration.
//!
//! Based on "Recursive Language Models" (Zhang et al. 2025)

// Core types from the workspace crate
pub use codetether_rlm::*;

// Compatibility exports and application context adapters.
pub use codetether_rlm::router_executor as executor_bridge;
pub use codetether_rlm_legacy as repl;
pub mod router;
pub use codetether_rlm::tools;

// Re-export RLM entry points at the rlm:: level for existing call sites.
pub use executor_bridge::RlmExecutor;
pub use router::RlmRouter;
