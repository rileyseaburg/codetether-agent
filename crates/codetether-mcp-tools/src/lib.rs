//! Adapt remote MCP tools to the shared local tool interface.
//!
//! [McpToolManager] lists remote tools and creates [McpToolWrapper] adapters.
//! [Client] retains connection and invocation authorization in the host client;
//! [convert] preserves text, resources, errors and images from MCP responses.
mod client;
mod connect;
pub mod convert;
mod manager;
mod wrapper;
mod wrapper_clone;
mod wrapper_tool;
pub use client::Client;
pub use manager::McpToolManager;
pub use wrapper::McpToolWrapper;
