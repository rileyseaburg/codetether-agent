//! Pareto evaluation harness for `DerivePolicy`.
//!
//! ## Role (Phase B step 23)
//!
//! Liu et al. (arXiv:2512.22087), Lu et al. (arXiv:2510.06727), and
//! ClawVM (arXiv:2604.10352) agree on one methodological point: a
//! policy is never flipped on aggregate accuracy alone. Pareto over
//! `(accuracy, context-cost, reuse-rate, oracle-gap)` is the minimum.
//! This module is the *comparator* half of that evaluation loop. The
//! *runner* half — actually executing `derive_with_policy` over a
//! fixture corpus and collecting [`PolicyRunResult`]s — is a
//! follow-up commit.
//!
//! ## Scope
//!
//! * [`PolicyRunResult`] — per-policy datapoint.
//! * [`pareto_frontier`] — drop dominated points, keep frontier.
//! * [`reuse_rate`] — Cognitive-Workspace-style warm-hit proxy.
//!
//! The ClawVM Tier-1 fault regression suite is a sibling concern;
//! those tests live as `#[test]` functions on the subsystems they
//! exercise (compression, journal, session_recall) rather than as
//! harness inputs.
//!
//! ## Examples
//!
//! ```rust
//! use codetether_policy_eval::{PolicyRunResult, pareto_frontier, reuse_rate};
//!
//! let results = vec![
//!     PolicyRunResult {
//!         policy: "legacy",
//!         kept_messages: 30,
//!         context_tokens: 24_000,
//!         fault_count: 12,
//!         oracle_gap: 4,
//!         reuse_rate: 0.50,
//!     },
//!     PolicyRunResult {
//!         policy: "reset",
//!         kept_messages: 8,
//!         context_tokens: 6_500,
//!         fault_count: 5,
//!         oracle_gap: 2,
//!         reuse_rate: 0.62,
//!     },
//! ];
//! let frontier = pareto_frontier(&results);
//! assert!(frontier.iter().any(|r| r.policy == "reset"));
//!
//! // 4 of 8 context entries were already warm from the prior turn.
//! assert!((reuse_rate(&[4, 8]) - 0.5).abs() < 1e-9);
//! ```

mod dominance;
mod frontier;
mod reuse;
mod sample;
pub use frontier::pareto_frontier;
pub use reuse::reuse_rate;
pub use sample::PolicyRunResult;
#[cfg(test)]
mod tests;
