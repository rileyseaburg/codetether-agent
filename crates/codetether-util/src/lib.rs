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
//! use codetether_util::truncate_bytes_safe;
//!
//! assert_eq!(truncate_bytes_safe("hello", 3), "hel");
//! assert_eq!(truncate_bytes_safe("aé", 2), "a");
//! ```

pub mod truncate;
pub use truncate::truncate_bytes_safe;
pub mod workspace_scan;

/// Case-insensitive subsequence ranking.
pub mod fuzzy;
#[cfg(test)]
mod fuzzy_tests;

/// UTF-8 safe text excerpts limited by line and byte budgets.
pub mod text_preview;

/// Character-count limited display previews.
pub mod preview_chars;

/// Environment-value parsing shared by startup and policy adapters.
pub mod environment;
