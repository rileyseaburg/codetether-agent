//! Native specialization of shared thinker client ownership and dispatch.
pub(in crate::cognition) use codetether_native_providers::thinker::Services;
pub use codetether_native_providers::thinker::ThinkerClient;
#[cfg(test)]
#[path = "client/native_tests.rs"]
mod native_tests;
