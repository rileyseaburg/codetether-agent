//! A2A server using the application session and native provider services.
mod application;
/// A2A server with native session and provider execution.
pub type A2AServer = codetether_native_a2a::server::A2AServer<crate::session::Session>;
