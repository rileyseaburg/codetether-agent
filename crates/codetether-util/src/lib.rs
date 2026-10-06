//! # CodeTether shared utilities
//!
//! Dependency-free helpers shared by CodeTether crates:
//!
//! - [`truncate`] — UTF-8 safe byte truncation.
//! - [`workspace_scan`] — directory pruning rules for workspace walkers.
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
