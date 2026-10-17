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
