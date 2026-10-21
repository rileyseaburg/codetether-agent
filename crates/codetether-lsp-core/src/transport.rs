//! LSP transport layer - stdio implementation with Content-Length framing
//!
//! LSP uses a special framing format with Content-Length headers:
//! ```text
//! Content-Length: 123\r\n
//! \r\n
//! <JSON payload>
//! ```

use super::types::{JsonRpcNotification, JsonRpcRequest, JsonRpcResponse};

use anyhow::Result;

use process::ServerProcess as Child;

use std::collections::HashMap;

use std::sync::Arc;

use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};

use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};

use tokio::sync::{RwLock, mpsc, oneshot};

use tracing::{debug, error, trace, warn};

#[path = "transport_message.rs"]
mod message;

#[path = "transport_process.rs"]
mod process;

/// Maximum allowed value for the LSP `Content-Length` header.
///
/// LSP messages are typically well under 1 MiB. A corrupted or buggy server
/// could emit an absurd value, causing `vec![0u8; len]` to request more memory
/// than the process has — tripping the alloc guard.
const MAX_CONTENT_LENGTH: usize = 16 * 1024 * 1024;

// 16 MiB

/// LSP Transport for communicating with language servers
pub struct LspTransport {
    /// The child process (kept alive for the transport lifetime)
    _child: Child,
    /// Channel for sending messages
    tx: mpsc::Sender<String>,
    /// Pending requests waiting for responses
    pending: Arc<RwLock<HashMap<i64, oneshot::Sender<JsonRpcResponse>>>>,
    /// Request ID counter
    request_id: AtomicI64,
    /// Whether the server is initialized
    initialized: std::sync::atomic::AtomicBool,
    /// Per-request timeout in milliseconds.
    timeout_ms: u64,
    /// Recent stderr lines from the language server for diagnostics.
    recent_stderr: Arc<RwLock<Vec<String>>>,
    /// Server command for diagnostics.
    command: String,
    /// Last diagnostics published by the language server, keyed by URI.
    diagnostics: Arc<RwLock<HashMap<String, Vec<lsp_types::Diagnostic>>>>,
    /// Monotonic counter bumped every time the server publishes diagnostics
    /// for any URI. Used by callers to detect fresh publications after a
    /// `textDocument/didChange` so stale cached diagnostics don't leak into
    /// post-edit validation.
    diag_publish_seq: Arc<AtomicU64>,
    /// Lifetime-bound publisher for the existing diagnostic cache.
    sharing: std::sync::Mutex<Option<sharing::Sharing>>,
    /// End-of-stream liveness prevents refreshing a dead server's diagnostics.
    reader_alive: Arc<std::sync::atomic::AtomicBool>,
}

mod diagnostics;
mod headers;
mod notify;
mod reader;
mod request;
mod sharing;
mod spawn;
mod state;
mod stderr;
mod writer;

mod frame;
