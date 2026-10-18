//! Agent-tool contract, input hydration and action routing.
//!
//! Construct [AgentTool] with an application [Host]. The tool parses the shared
//! payload, hydrates its parent scope and dispatches existing agent actions.
//! The schema and legacy action/error contracts remain with this entrypoint.
mod actions;
mod dispatch;
mod host;
mod schema;
mod tool;
mod tool_impl;
pub use dispatch::execute as dispatch;
pub use host::Host;
pub use tool::AgentTool;

mod application_binding;

/// Dependencies used by application-local agent-tool declarations.
#[doc(hidden)]
pub mod __private {
    pub use anyhow::Result;
    pub use async_trait::async_trait;
    pub use codetether_agent_input::Params;
    pub use codetether_agent_lifecycle::ResumeConfig;
    pub use codetether_tool_core::ToolResult;
}
