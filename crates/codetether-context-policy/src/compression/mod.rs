//! Budget-driven emergency compression policies.
//!
//! [terminal] shrinks payloads and evicts old messages after summarization
//! has been exhausted. The host supplies token estimates and diagnostics.
pub mod terminal;

/// Summary installation, cached-message rendering and dropped-range indexes.
pub mod summary;

/// Request-budget triggers and keep-last cascade decisions.
pub mod budget;
