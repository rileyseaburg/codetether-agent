//! Authenticated client for the inherited mux worktree coordinator.
//!
//! [active] detects inherited mux identity; [acquire], [renew] and [release]
//! discover the authority, exchange typed messages and publish advisory bus events.
//! [SESSION_ENV] names the shared inherited-session environment variable.
mod api;
mod client;
mod event;
mod operation;
pub use api::{SESSION_ENV, acquire, active, release, renew};
