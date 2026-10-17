//! Shared MCP protocol contracts, message classification and tool-result conversion.
//!
//! Runtime transport, permissions and registry wiring belong to callers.
//!
//! # Examples
//! ```
//! let request = codetether_mcp_core::types::JsonRpcRequest::new(1i64, "tools/list", None);
//! assert_eq!(request.jsonrpc, "2.0");
//! assert_eq!(request.method, "tools/list");
//! ```
mod message;
pub mod tool_result;
pub mod types;
pub use message::McpMessage;

/// Client-generic registry with capability metadata and remote tool routing.
pub mod registry;

/// Server handler registration, metadata and protocol catalog queries.
pub mod server_catalog;
