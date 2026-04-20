//! LSP transport layer - stdio implementation with Content-Length framing
//!
//! LSP uses a special framing format with Content-Length headers:
//! ```text
//! Content-Length: 123\r\n
//! \r\n
//! <JSON payload>
//! ```

use super::types::{JsonRpcNotification, JsonRpcRequest, JsonRpcResponse};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicI64, AtomicU64, Ordering};
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
}

impl LspTransport {
    /// Spawn a language server and create a transport
    pub async fn spawn(command: &str, args: &[String], timeout_ms: u64) -> Result<Self> {
        let mut child = Command::new(command)
            .args(args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .with_context(|| format!("Failed to spawn language server '{command}'"))?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow::anyhow!("No stdout"))?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| anyhow::anyhow!("No stderr"))?;
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| anyhow::anyhow!("No stdin"))?;

        let (write_tx, mut write_rx) = mpsc::channel::<String>(100);
        let pending: Arc<RwLock<HashMap<i64, oneshot::Sender<JsonRpcResponse>>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let recent_stderr = Arc::new(RwLock::new(Vec::new()));
        let diagnostics = Arc::new(RwLock::new(HashMap::new()));
        let diag_publish_seq = Arc::new(AtomicU64::new(0));

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
            pending_clone.write().await.clear();
        });

        // Stderr task - capture recent diagnostics from the language server.
        let recent_stderr_clone = Arc::clone(&recent_stderr);
        let stderr_command = command.to_string();
        tokio::spawn(async move {
            let mut reader = BufReader::new(stderr);
            let mut line = String::new();
            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) => return,
                    Ok(_) => {
                        let trimmed = line.trim().to_string();
                        if trimmed.is_empty() {
                            continue;
                        }
                        warn!(command = %stderr_command, stderr = %trimmed, "Language server stderr");
                        let mut guard = recent_stderr_clone.write().await;
                        guard.push(trimmed);
                        if guard.len() > 20 {
                            let excess = guard.len() - 20;
                            guard.drain(0..excess);
                        }
                    }
                    Err(e) => {
                        warn!(command = %stderr_command, error = %e, "Failed reading language server stderr");
                        return;
                    }
                }
            }
        });

        // Reader task - parses Content-Length framed responses and notifications.
        let pending_clone = Arc::clone(&pending);
        let diagnostics_clone = Arc::clone(&diagnostics);
        let diag_publish_seq_clone = Arc::clone(&diag_publish_seq);
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout);
            let mut header_buf = String::new();

            loop {
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
                                break;
                            }
                            if let Some(stripped) = line.strip_prefix("Content-Length:")
                                && let Ok(len) = stripped.trim().parse::<usize>()
                            {
                                content_length = Some(len);
                            }
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

                let mut body_buf = vec![0u8; len];
                match reader.read_exact(&mut body_buf).await {
                    Ok(_) => {
                        let body = String::from_utf8_lossy(&body_buf);
                        trace!("LSP RX: {}", body);

                        if let Ok(response) = serde_json::from_str::<JsonRpcResponse>(&body) {
                            let mut pending_guard = pending_clone.write().await;
                            if let Some(tx) = pending_guard.remove(&response.id) {
                                let id = response.id;
                                if tx.send(response).is_err() {
                                    warn!("Request {} receiver dropped", id);
                                }
                            } else {
                                debug!("Received response for unknown request {}", response.id);
                            }
                            continue;
                        }

                        match serde_json::from_str::<serde_json::Value>(&body) {
                            Ok(value) => {
                                if value.get("method").and_then(serde_json::Value::as_str)
                                    == Some("textDocument/publishDiagnostics")
                                {
                                    if let Some(params) = value.get("params") {
                                        let uri = params
                                            .get("uri")
                                            .and_then(serde_json::Value::as_str)
                                            .unwrap_or_default()
                                            .to_string();
                                        let diagnostics = params
                                            .get("diagnostics")
                                            .cloned()
                                            .and_then(|v| serde_json::from_value(v).ok())
                                            .unwrap_or_default();
                                        if !uri.is_empty() {
                                            diagnostics_clone
                                                .write()
                                                .await
                                                .insert(uri, diagnostics);
                                            diag_publish_seq_clone
                                                .fetch_add(1, Ordering::SeqCst);
                                        }
                                    }
                                } else {
                                    debug!(
                                        "Ignoring LSP notification/message without tracked handler: {}",
                                        body
                                    );
                                }
                            }
                            Err(e) => {
                                debug!("Failed to parse LSP message: {} - body: {}", e, body);
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
            timeout_ms,
            recent_stderr,
            command: command.to_string(),
            diagnostics,
            diag_publish_seq,
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

        let response = tokio::time::timeout(std::time::Duration::from_millis(self.timeout_ms), rx)
            .await
            .map_err(|_| {
                let stderr_summary = self.stderr_summary();
                anyhow::anyhow!(
                    "LSP request timeout for method: {} (server: {}, timeout: {}ms{})",
                    method,
                    self.command,
                    self.timeout_ms,
                    stderr_summary
                        .as_deref()
                        .map(|summary| format!(", recent stderr: {summary}"))
                        .unwrap_or_default()
                )
            })?
            .map_err(|_| anyhow::anyhow!("LSP response channel closed"))?;

        Ok(response)
    }

    fn stderr_summary(&self) -> Option<String> {
        self.recent_stderr.try_read().ok().and_then(|lines| {
            if lines.is_empty() {
                None
            } else {
                Some(lines.join(" | "))
            }
        })
    }

    /// Send a notification (no response expected)
    pub async fn notify(&self, method: &str, params: Option<serde_json::Value>) -> Result<()> {
        let notification = JsonRpcNotification::new(method, params);
        let json = serde_json::to_string(&notification)?;
        self.tx.send(json).await?;
        Ok(())
    }

    /// Return the last diagnostics published by the language server.
    pub async fn diagnostics_snapshot(&self) -> HashMap<String, Vec<lsp_types::Diagnostic>> {
        self.diagnostics.read().await.clone()
    }

    /// Current publish sequence counter. Increments every time the language
    /// server publishes diagnostics for any URI. Callers can read this before
    /// a `textDocument/didChange` and then wait via
    /// [`Self::wait_for_publish_after`] for the server to republish.
    pub fn diagnostics_publish_seq(&self) -> u64 {
        self.diag_publish_seq.load(Ordering::SeqCst)
    }

    /// Remove any cached diagnostics for `uri`. Useful before a didChange so
    /// stale entries can't be returned while the server recomputes.
    pub async fn invalidate_diagnostics(&self, uri: &str) {
        self.diagnostics.write().await.remove(uri);
    }

    /// Wait until the server publishes diagnostics with a sequence greater
    /// than `baseline`, or the timeout elapses. Returns `true` if a new
    /// publication arrived in time.
    pub async fn wait_for_publish_after(
        &self,
        baseline: u64,
        timeout: std::time::Duration,
    ) -> bool {
        let deadline = std::time::Instant::now() + timeout;
        loop {
            if self.diag_publish_seq.load(Ordering::SeqCst) > baseline {
                return true;
            }
            if std::time::Instant::now() >= deadline {
                return false;
            }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
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
