//! MCP transports with separate standard-I/O, HTTP and subprocess ownership.
//!
//! Callers authorize connections before constructing a transport. Transport
//! methods exchange the shared MCP JSON-RPC records and preserve framing.
use anyhow::Result;
use async_trait::async_trait;
pub use codetether_mcp_core::McpMessage;
use codetether_mcp_core::types::*;
use serde_json::Value;
use std::io::{BufRead, Write};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc;
use tracing::{debug, error, trace, warn};
mod contract;
mod null;
mod process;
mod sse;
mod stdio;
pub use contract::Transport;
pub use null::NullTransport;
pub use process::ProcessTransport;
pub use sse::SseTransport;
pub use stdio::StdioTransport;

/// Request IDs, pending response channels and JSON-RPC transactions.
pub mod rpc;

/// Remote MCP initialization, tool, resource and prompt operations.
pub mod remote;

/// Remote MCP protocol session and cached peer metadata.
pub mod client;
