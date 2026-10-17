//! Agent definitions, prompt composition, and execution-result contracts.
//!
//! [AgentRegistry] owns the metadata catalog. [builtin] composes prompts and
//! discovers project instructions without depending on application sessions,
//! tools, or swarm execution. The application owns running agents.
//!
//! ```
//! use codetether_agent_core::AgentRegistry;
//! assert_eq!(AgentRegistry::with_builtins().list().len(), 3);
//! ```
pub mod build_guidance;
pub mod builtin;
mod catalog;
mod types;
pub use catalog::AgentRegistry;
pub use types::{AgentInfo, AgentMode, AgentResponse, ToolMetadata, ToolUse};
pub mod events;
#[cfg(test)]
mod tests;

/// Shared message handling and tool-execution adapters.
pub mod execution;
