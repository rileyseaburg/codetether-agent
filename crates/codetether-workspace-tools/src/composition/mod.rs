//! Workspace catalog construction and swarm coordination extensions.
//!
//! Implement [Factory], then use [with_provider] for an isolated catalog,
//! [for_agent] for main-swarm coordination, or [direct::shared] for direct workers.
mod catalog;
pub mod direct;
mod factory;
mod local;
mod share;
pub use catalog::{with_defaults, with_provider};
pub use factory::Factory;
pub use local::for_agent;
pub use share::with_shared;
