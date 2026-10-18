//! Native A2A discovery, session runtime and credential integration.
//!
//! [mdns] preserves the application version advertised during discovery.
//! [git_credentials] binds credential-helper arguments to the shared broker.
//! [intro] and [stream] expose the canonical protocol services.
pub mod git_credentials;
pub mod intro;
pub mod mdns;
pub mod stream;

mod session;
pub use session::Session;
/// Headless A2A prompting with the native shared provider registry.
pub mod prompt_runtime;
/// Native session/provider binding for the shared A2A server.
pub mod server;
/// Default-model configuration and best-effort session persistence.
pub mod session_config;
