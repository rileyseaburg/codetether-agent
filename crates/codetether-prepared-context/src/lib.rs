//! Prepared summary-index storage and transcript compatibility checks.
//!
//! [Prepared] stores a versioned summary index and source-prefix fingerprint.
//! [read], [write] and [remove] operate on paths supplied by the caller after
//! native validation. [window_index] projects durable ranges into a live window.
mod fingerprint;
mod freshness;
mod prepared;
mod read;
mod window;
mod write;
pub use fingerprint::fingerprint;
pub use freshness::matches;
pub use prepared::Prepared;
pub use read::{decode, read};
pub use window::window_index;
pub use write::{remove, write};
#[cfg(test)]
mod tests;

/// Implementation support for application-local persistence bindings.
#[doc(hidden)]
pub mod session_store;
