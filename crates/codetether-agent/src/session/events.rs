//! Native specialization of shared session events and results.
pub use codetether_session_events::{SessionResult, StreamRetryEvent};
/// Streaming events carrying native session snapshots.
pub type SessionEvent = codetether_session_events::SessionEvent<super::Session>;
#[cfg(test)]
mod durable_tests;
