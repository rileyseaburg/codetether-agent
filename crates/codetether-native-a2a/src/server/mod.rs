//! Native provider and session services for the shared A2A server.
mod application;
/// Shared A2A server specialized to application session type `S`.
#[allow(type_alias_bounds)]
pub type A2AServer<S: crate::Session> = codetether_a2a_server::A2AServer<application::Native<S>>;
