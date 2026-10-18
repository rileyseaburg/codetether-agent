//! Native A2A discovery, introductions and credential integration.
//!
//! [mdns] preserves the application version advertised during discovery.
//! [git_credentials] binds credential-helper arguments to the shared broker.
//! [intro] and [stream] expose the canonical protocol services.
pub mod git_credentials;
pub mod intro;
pub mod mdns;
pub mod stream;
