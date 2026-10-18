//! # CodeTether shared utilities
//!
//! Dependency-free helpers shared by CodeTether crates:
//!
//! - [`truncate`] — UTF-8 safe byte truncation.
//! - [`workspace_scan`] — directory pruning rules for workspace walkers.
//! - [`fuzzy`] — case-insensitive subsequence scores for ranked pickers.
//!
//! # Examples
//!
//! ```rust
//! use codetether_util::truncate::truncate_bytes_safe;
//!
//! assert_eq!(truncate_bytes_safe("hello", 3), "hel");
//! ```

pub mod truncate;
pub mod workspace_scan;

/// Case-insensitive subsequence ranking.
pub mod fuzzy;
#[cfg(test)]
mod fuzzy_tests;
