//! In-memory profile catalog and built-in profile definitions.
//!
//! [AgentRegistry] stores, replaces, and filters agent metadata. Built-in
//! constructors are also exposed by the public builtin module.
mod definitions;
mod registry;
pub use definitions::{build_agent, explore_agent, plan_agent};
pub use registry::AgentRegistry;
