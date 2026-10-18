//! Bounded Ralph story and conflict-resolver agent execution.
//!
//! Implement [Host] for native prompt/tool construction and agent-loop access.
//! [direct] retains its event bridge; [parallel] uses the caller's bridge and
//! step limit; [conflict] preserves the smaller conflict-resolver budget.
mod bridge;
mod conflict;
mod direct;
mod exit;
mod host;
mod parallel;
mod prepare;
mod request;
pub use bridge::event_bridge;
pub use conflict::conflict;
pub use direct::direct;
pub use host::Host;
pub use parallel::parallel;
pub use request::Invocation;
