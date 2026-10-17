//! Fallback tool registration when no application registry is attached.
//!
//! Tool modules own schemas and register in the established order. File
//! handlers preserve their existing reads, writes and approval checks.
use super::{Arc, CallToolResult, McpServer, Result, ToolContent, Value, json};
mod grep_search;
mod list_directory;
mod list_directory_handler;
mod read_file;
mod read_file_handler;
mod run_command;
mod search_files;
mod write_file;
mod write_file_handler;
pub(super) async fn register(server: &McpServer) {
    run_command::register(server).await;
    read_file::register(server).await;
    write_file::register(server).await;
    list_directory::register(server).await;
    search_files::register(server).await;
    grep_search::register(server).await;
    tracing::info!(target: "codetether_agent::mcp::server", "Registered {} MCP tools", server.catalog.tools().read().await.len());
}
