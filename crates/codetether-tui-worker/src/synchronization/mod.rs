//! Nonblocking synchronization of UI worker state with its bridge.
//!
//! [sync_agents] derives registrations from observed READY messages.
//! [set_processing] records and reports a processing transition; [send] exposes
//! the same optional-sender behavior for other bridge commands.
mod agents;
mod processing;
mod send;
pub use agents::sync_agents;
pub use processing::set_processing;
pub use send::send;
