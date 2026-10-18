//! Native MCP client authorization and application tool adapters.
//!
//! [McpClient] and [McpRegistry] enforce process and tool policy. [mcp_bridge]
//! and [mcp_tools] expose those clients as application tools. Server adapters
//! can use [approval], [tool_policy] and [fallback_run_command] without depending
//! on a concrete application registry.
mod client;
pub use client::{McpClient, McpRegistry};
#[doc(hidden)]
pub mod approval;
#[doc(hidden)]
pub mod fallback_run_command;
mod policy_host;
mod subprocess_policy;
#[doc(hidden)]
pub mod tool_policy;
pub mod transport;
use codetether_mcp_core::types;
pub use types::*;
pub mod mcp_bridge;
pub mod mcp_tools;
pub use mcp_tools::{McpToolManager, McpToolWrapper};
