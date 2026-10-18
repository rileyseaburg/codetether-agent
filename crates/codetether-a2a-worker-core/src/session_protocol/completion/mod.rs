//! Worker completion fallback policy over host-owned request preparation.
//!
//! [complete_with_fallback] makes the initial attempt and at most one
//! compact-schema retry through [CompletionHost].
mod host;
mod run;
pub use host::CompletionHost;
pub use run::complete_with_fallback;

/// Goal-aware completion with model temperature and usage accounting.
pub mod accounted;
