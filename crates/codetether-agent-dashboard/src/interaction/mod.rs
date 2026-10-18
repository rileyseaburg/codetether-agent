//! Enter-key selection for the unified agent dashboard.
//!
//! Implement [Host] for native registry and focus operations, then call [enter].
//! Stale focus is removed before choosing local or swarm detail views.
mod binding;
mod enter;
mod focus;
mod host;
mod swarm;
pub use enter::enter;
pub use host::Host;
