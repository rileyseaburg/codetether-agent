//! Shared workspace summary and attention records.
//!
//! [GlobalWorkspace] stores selected beliefs and objectives. [AttentionItem]
//! identifies work requiring persona focus with an [AttentionSource].
mod attention;
mod state;
pub use attention::{AttentionItem, AttentionSource};
pub use state::GlobalWorkspace;

#[cfg(test)]
mod tests;

/// Selection policy for workspace summaries.
pub mod ranking;

/// Shared workspace refresh and event construction.
pub mod refresh;
