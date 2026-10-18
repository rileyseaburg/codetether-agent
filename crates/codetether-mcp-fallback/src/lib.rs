//! Fallback tool registration when no application registry is attached.
//!
//! Tool modules own schemas and register in the established order. File
//! handlers preserve their existing reads, writes and approval checks.
//! Implement [Host] and pass an existing catalog to [register].
use anyhow::Result;
use codetether_mcp_core::{
    server_catalog::ServerCatalog,
    types::{CallToolResult, ToolContent},
};
use serde_json::{Value, json};
use std::sync::Arc;
mod host;
pub use host::Host;
mod grep_search;
mod list_directory;
mod list_directory_handler;
mod read_file;
mod read_file_handler;
mod run_command;
mod search_files;
mod write_file;
mod write_file_handler;
/// Register the six fallback tools in the established order.
/// # Arguments
/// * `server` - Existing MCP catalog receiving schemas and handlers.
/// # Returns
/// Registers command, read, write, directory, file-search and grep handlers.
/// # Examples
/// ```no_run
/// async fn setup<H: codetether_mcp_fallback::Host>(catalog: &codetether_mcp_core::server_catalog::ServerCatalog) {
///     codetether_mcp_fallback::register::<H>(catalog).await;
/// }
/// ```
pub async fn register<H: Host>(server: &ServerCatalog) {
    run_command::register::<H>(server).await;
    read_file::register(server).await;
    write_file::register::<H>(server).await;
    list_directory::register(server).await;
    search_files::register::<H>(server).await;
    grep_search::register::<H>(server).await;
    tracing::info!(target: "codetether_agent::mcp::server", "Registered {} MCP tools", server.tools().read().await.len());
}

mod command;
/// Canonical runtime policy aliases for fallback tool names.
pub mod policy;
