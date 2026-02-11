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
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, Ordering};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::{RwLock, mpsc, oneshot};
use tracing::{debug, error, trace, warn};

/// LSP Transport for communicating with language servers
pub struct LspTransport {
    /// The child process (kept alive for the transport lifetime)
    _child: Child,
    /// Channel for sending messages
    tx: mpsc::Sender<String>,
    /// Pending requests waiting for responses
    pending: Arc<RwLock<std::collections::HashMap<i64, oneshot::Sender<JsonRpcResponse>>>>,
    /// Request ID counter
    request_id: AtomicI64,
    /// Whether the server is initialized
    initialized: std::sync::atomic::AtomicBool,
}

impl LspTransport {
    /// Spawn a language server and create a transport
    pub async fn spawn(command: &str, args: &[String]) -> Result<Self> {
        let mut child = Command::new(command)
            .args(args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::inherit())
            .spawn()?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow::anyhow!("No stdout"))?;
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| anyhow::anyhow!("No stdin"))?;

        let (write_tx, mut write_rx) = mpsc::channel::<String>(100);
        let pending: Arc<RwLock<std::collections::HashMap<i64, oneshot::Sender<JsonRpcResponse>>>> =
            Arc::new(RwLock::new(std::collections::HashMap::new()));

        // Writer task - sends messages with Content-Length framing
        let pending_clone = Arc::clone(&pending);
        tokio::spawn(async move {
            while let Some(msg) = write_rx.recv().await {
                let content_length = msg.len();
                let header = format!("Content-Length: {}\r\n\r\n", content_length);
                trace!("LSP TX header: {}", header.trim());
                trace!("LSP TX body: {}", msg);

                if let Err(e) = stdin.write_all(header.as_bytes()).await {
                    error!("Failed to write header to LSP server: {}", e);
                    break;
                }
                if let Err(e) = stdin.write_all(msg.as_bytes()).await {
                    error!("Failed to write body to LSP server: {}", e);
                    break;
                }
                if let Err(e) = stdin.flush().await {
                    error!("Failed to flush LSP server stdin: {}", e);
                    break;
                }
            }
            // Clear pending requests on shutdown
            pending_clone.write().await.clear();
        });

        // Reader task - parses Content-Length framed responses
        let pending_clone = Arc::clone(&pending);
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout);
            let mut header_buf = String::new();

            loop {
                // Read headers until empty line
                header_buf.clear();
                let mut content_length: Option<usize> = None;

                loop {
                    header_buf.clear();
                    match reader.read_line(&mut header_buf).await {
                        Ok(0) => {
                            debug!("LSP server closed connection");
                            return;
                        }
                        Ok(_) => {
                            let line = header_buf.trim();
                            if line.is_empty() {
                                break; // End of headers
                            }
                            if let Some(stripped) = line.strip_prefix("Content-Length:") {
                                if let Ok(len) = stripped.trim().parse::<usize>() {
                                    content_length = Some(len);
                                }
                            }
                            // Ignore other headers (Content-Type, etc.)
                        }
                        Err(e) => {
                            error!("Failed to read header from LSP server: {}", e);
                            return;
                        }
                    }
                }

                let Some(len) = content_length else {
                    warn!("LSP message missing Content-Length header");
                    continue;
                };

                // Read the body
                let mut body_buf = vec![0u8; len];
                match reader.read_exact(&mut body_buf).await {
                    Ok(_) => {
                        let body = String::from_utf8_lossy(&body_buf);
                        trace!("LSP RX: {}", body);

                        // Parse as JSON-RPC response
                        match serde_json::from_str::<JsonRpcResponse>(&body) {
                            Ok(response) => {
                                // Find and complete the pending request
                                let mut pending_guard = pending_clone.write().await;
                                if let Some(tx) = pending_guard.remove(&response.id) {
                                    let id = response.id;
                                    if tx.send(response).is_err() {
                                        warn!("Request {} receiver dropped", id);
                                    }
                                } else {
                                    // Could be a response to a notification or unknown request
                                    debug!("Received response for unknown request {}", response.id);
                                }
                            }
                            Err(e) => {
                                // Might be a notification (no id) or malformed
                                debug!("Failed to parse LSP response: {} - body: {}", e, body);
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to read LSP message body: {}", e);
                        return;
                    }
                }
            }
        });

        Ok(Self {
            _child: child,
            tx: write_tx,
            pending,
            request_id: AtomicI64::new(1),
            initialized: std::sync::atomic::AtomicBool::new(false),
        })
    }

    /// Send a request and wait for response
    pub async fn request(
        &self,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> Result<JsonRpcResponse> {
        let id = self.request_id.fetch_add(1, Ordering::SeqCst);
        let request = JsonRpcRequest::new(id, method, params);

        let (tx, rx) = oneshot::channel();
        self.pending.write().await.insert(id, tx);

        let json = serde_json::to_string(&request)?;
        self.tx.send(json).await?;

        // Wait for response with timeout
        let response = tokio::time::timeout(std::time::Duration::from_secs(30), rx)
            .await
            .map_err(|_| anyhow::anyhow!("LSP request timeout for method: {}", method))?
            .map_err(|_| anyhow::anyhow!("LSP response channel closed"))?;

        Ok(response)
    }

    /// Send a notification (no response expected)
    pub async fn notify(&self, method: &str, params: Option<serde_json::Value>) -> Result<()> {
        let notification = JsonRpcNotification::new(method, params);
        let json = serde_json::to_string(&notification)?;
        self.tx.send(json).await?;
        Ok(())
    }

    /// Check if the server is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized.load(std::sync::atomic::Ordering::SeqCst)
    }

    /// Mark the server as initialized
    pub fn set_initialized(&self, value: bool) {
        self.initialized
            .store(value, std::sync::atomic::Ordering::SeqCst);
    }
}

impl Drop for LspTransport {
    fn drop(&mut self) {
        if self.is_initialized() {
            tracing::debug!("LspTransport dropped while still initialized");
        }
    }
}
