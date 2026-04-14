//! Agent system.
//!
//! Agents orchestrate providers, tools, and swarm participation. The root
//! module wires focused submodules together and re-exports the public types.
//!
//! # Examples
//!
//! ```ignore
//! let registry = codetether::agent::AgentRegistry::with_builtins();
//! assert!(!registry.list().is_empty());
//! ```

pub mod build_guidance;
#[path = "builtin/mod.rs"]
pub mod builtin;
mod core;
mod execution;
mod registry;
mod swarm;
mod tooling;
mod types;

#[cfg(test)]
mod tests;

pub use core::Agent;
pub use registry::AgentRegistry;
pub use types::{AgentInfo, AgentMode, AgentResponse, ToolMetadata, ToolUse};
