//! Compatibility exports for discovery-owned peer routes.
pub(crate) use codetether_a2a_discovery::routes::{PeerRoute, get, list, register, remove};
#[cfg(test)]
#[path = "peer_route_tests.rs"]
mod tests;
