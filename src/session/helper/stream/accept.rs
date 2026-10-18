//! Shared final stream acceptance policy.
pub(super) use codetether_stream_outcome::{accept, exhausted};
#[cfg(test)]
#[path = "fault_accept_tests.rs"]
mod tests;
