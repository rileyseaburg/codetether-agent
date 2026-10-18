//! Incremental context projection around a prepared summary index.
//!
//! [Source] supplies immutable messages, page classification and prepared
//! summaries. [derive_incremental] protects required entries, packs available
//! summaries and applies shared budget/repair policy.
mod below_budget;
mod clamp;
mod derive;
mod pack;
mod required;
mod source;
pub use below_budget::try_pass_through;
pub use clamp::clamp_and_recompute;
pub use derive::derive_incremental;
pub use pack::select;
pub use required::seed;
pub use source::Source;

/// Implementation support for application-local session bindings.
#[doc(hidden)]
pub mod session_binding;

#[cfg(test)]
mod native_tests;
