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
