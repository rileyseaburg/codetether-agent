//! A2A server bound to native session and provider services.
mod application;
/// A2A server using the application session runtime.
pub type A2AServer = codetether_a2a_server::A2AServer<application::Native>;
