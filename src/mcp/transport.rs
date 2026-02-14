//! MCP transport layer - stdio and SSE implementations

use super::types::*;
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;
use std::io::{BufRead, Write};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc;
use tracing::{debug, error, trace, warn};

/// Transport trait for MCP communication
#[async_trait]
pub trait Transport: Send + Sync {
    /// Send a JSON-RPC request
    async fn send_request(&self, request: JsonRpcRequest) -> Result<()>;

    /// Send a JSON-RPC response
    async fn send_response(&self, response: JsonRpcResponse) -> Result<()>;

    /// Send a JSON-RPC notification
    async fn send_notification(&self, notification: JsonRpcNotification) -> Result<()>;

    /// Receive incoming messages
    async fn receive(&self) -> Result<Option<McpMessage>>;

    /// Close the transport
    async fn close(&self) -> Result<()>;
}

/// Incoming MCP message (can be request, response, or notification)
#[derive(Debug, Clone)]
pub enum McpMessage {
    Request(JsonRpcRequest),
    Response(JsonRpcResponse),
    Notification(JsonRpcNotification),
}

impl McpMessage {
    pub fn from_json(value: Value) -> Result<Self> {
        // Check if it has an id
        if value.get("id").is_some() {
            // Has id - could be request or response
            if value.get("method").is_some() {
                // Has method - it's a request
                let request: JsonRpcRequest = serde_json::from_value(value)?;
                Ok(McpMessage::Request(request))
            } else {
                // No method - it's a response
                let response: JsonRpcResponse = serde_json::from_value(value)?;
                Ok(McpMessage::Response(response))
            }
        } else {
            // No id - it's a notification
            let notification: JsonRpcNotification = serde_json::from_value(value)?;
            Ok(McpMessage::Notification(notification))
        }
    }
}

/// Stdio transport for MCP (synchronous version for server mode)
pub struct StdioTransport {
    /// Sender channel for outgoing messages (kept alive for transport lifetime)
    #[allow(dead_code)]
    tx: mpsc::Sender<String>,
    rx: tokio::sync::Mutex<mpsc::Receiver<String>>,
}

/// Null transport for local/in-process MCP usage.
///
/// This transport intentionally does **not** spawn any stdio reader/writer threads
/// and does not lock stdout. It is suitable for CLI commands that want to reuse the
/// MCP tool registry (e.g. `codetether mcp list-tools` and `codetether mcp call`)
/// without running a stdio server.
#[derive(Debug, Default, Clone)]
pub struct NullTransport;

impl NullTransport {
    pub fn new() -> Self {
        Self
    }
}

impl Default for StdioTransport {
    fn default() -> Self {
        Self::new()
    }
}

impl StdioTransport {
    /// Create a new stdio transport
    pub fn new() -> Self {
        let (write_tx, mut write_rx) = mpsc::channel::<String>(100);
        let (read_tx, read_rx) = mpsc::channel::<String>(100);

        // Spawn writer thread (blocking IO)
        std::thread::spawn(move || {
            let mut stdout = std::io::stdout().lock();
            while let Some(msg) = write_rx.blocking_recv() {
                trace!("MCP TX: {}", msg);
                if let Err(e) = writeln!(stdout, "{}", msg) {
                    error!("Failed to write to stdout: {}", e);
                    break;
                }
                if let Err(e) = stdout.flush() {
                    error!("Failed to flush stdout: {}", e);
                    break;
                }
            }
        });

        // Spawn reader thread (blocking IO)
        std::thread::spawn(move || {
            let stdin = std::io::stdin();
            let reader = stdin.lock();
            for line in reader.lines() {
                match line {
                    Ok(msg) if !msg.is_empty() => {
                        trace!("MCP RX: {}", msg);
                        if read_tx.blocking_send(msg).is_err() {
                            break;
                        }
                    }
                    Ok(_) => continue, // Empty line
                    Err(e) => {
                        error!("Failed to read from stdin: {}", e);
                        break;
                    }
                }
            }
        });

        Self {
            tx: write_tx,
            rx: tokio::sync::Mutex::new(read_rx),
        }
    }

    async fn send_json(&self, value: Value) -> Result<()> {
        let json = serde_json::to_string(&value)?;
        self.tx.send(json).await?;
        Ok(())
    }
}

#[async_trait]
impl Transport for StdioTransport {
    async fn send_request(&self, request: JsonRpcRequest) -> Result<()> {
        self.send_json(serde_json::to_value(&request)?).await
    }

    async fn send_response(&self, response: JsonRpcResponse) -> Result<()> {
        self.send_json(serde_json::to_value(&response)?).await
    }

    async fn send_notification(&self, notification: JsonRpcNotification) -> Result<()> {
        self.send_json(serde_json::to_value(&notification)?).await
    }

    async fn receive(&self) -> Result<Option<McpMessage>> {
        let mut rx = self.rx.lock().await;
        match rx.recv().await {
            Some(line) => {
                let value: Value = serde_json::from_str(&line)?;
                let msg = McpMessage::from_json(value)?;
                Ok(Some(msg))
            }
            None => Ok(None),
        }
    }

    async fn close(&self) -> Result<()> {
        // Channel will close when transport is dropped
        Ok(())
    }
}

#[async_trait]
impl Transport for NullTransport {
    async fn send_request(&self, _request: JsonRpcRequest) -> Result<()> {
        Ok(())
    }

    async fn send_response(&self, _response: JsonRpcResponse) -> Result<()> {
        Ok(())
    }

    async fn send_notification(&self, _notification: JsonRpcNotification) -> Result<()> {
        Ok(())
    }

    async fn receive(&self) -> Result<Option<McpMessage>> {
        Ok(None)
    }

    async fn close(&self) -> Result<()> {
        Ok(())
    }
}

/// SSE transport for MCP (HTTP-based)
pub struct SseTransport {
    endpoint: String,
    client: reqwest::Client,
    _tx: mpsc::Sender<String>,
    rx: tokio::sync::Mutex<mpsc::Receiver<String>>,
}

impl SseTransport {
    /// Create a new SSE transport connecting to the given endpoint
    pub async fn new(endpoint: String) -> Result<Self> {
        let client = reqwest::Client::new();
        let (write_tx, _write_rx) = mpsc::channel::<String>(100);
        let (read_tx, read_rx) = mpsc::channel::<String>(100);

        // TODO: Start SSE connection for receiving messages
        // For now, this is a placeholder
        let endpoint_clone = endpoint.clone();
        let read_tx_clone = read_tx;

        tokio::spawn(async move {
            debug!("SSE transport connecting to: {}", endpoint_clone);
            // SSE event stream handling would go here
            let _ = read_tx_clone;
        });

        Ok(Self {
            endpoint,
            client,
            _tx: write_tx,
            rx: tokio::sync::Mutex::new(read_rx),
        })
    }

    async fn send_json(&self, value: Value) -> Result<()> {
        let json = serde_json::to_string(&value)?;
        debug!("SSE TX: {}", json);

        // POST to the endpoint
        self.client
            .post(&self.endpoint)
            .header("Content-Type", "application/json")
            .body(json)
            .send()
            .await?;

        Ok(())
    }
}

#[async_trait]
impl Transport for SseTransport {
    async fn send_request(&self, request: JsonRpcRequest) -> Result<()> {
        self.send_json(serde_json::to_value(&request)?).await
    }

    async fn send_response(&self, response: JsonRpcResponse) -> Result<()> {
        self.send_json(serde_json::to_value(&response)?).await
    }

    async fn send_notification(&self, notification: JsonRpcNotification) -> Result<()> {
        self.send_json(serde_json::to_value(&notification)?).await
    }

    async fn receive(&self) -> Result<Option<McpMessage>> {
        let mut rx = self.rx.lock().await;
        match rx.recv().await {
            Some(line) => {
                let value: Value = serde_json::from_str(&line)?;
                let msg = McpMessage::from_json(value)?;
                Ok(Some(msg))
            }
            None => Ok(None),
        }
    }

    async fn close(&self) -> Result<()> {
        Ok(())
    }
}

/// Process transport for connecting to MCP servers via subprocess
pub struct ProcessTransport {
    _child: tokio::process::Child,
    tx: mpsc::Sender<String>,
    rx: tokio::sync::Mutex<mpsc::Receiver<String>>,
}

impl ProcessTransport {
    /// Spawn a subprocess and connect via stdio
    pub async fn spawn(command: &str, args: &[&str]) -> Result<Self> {
        use tokio::process::Command;

        let mut child = Command::new(command)
            .args(args)
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow::anyhow!("No stdout"))?;
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| anyhow::anyhow!("No stdin"))?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| anyhow::anyhow!("No stderr"))?;

        let (write_tx, mut write_rx) = mpsc::channel::<String>(100);
        let (read_tx, read_rx) = mpsc::channel::<String>(100);

        // Stderr drain task — capture instead of inheriting so it doesn't corrupt the TUI
        tokio::spawn(async move {
            let mut reader = BufReader::new(stderr);
            let mut line = String::new();
            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) => break,
                    Ok(_) => {
                        let trimmed = line.trim();
                        if !trimmed.is_empty() {
                            warn!(target: "mcp_subprocess", "{trimmed}");
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        // Writer task
        tokio::spawn(async move {
            while let Some(msg) = write_rx.recv().await {
                trace!("Process TX: {}", msg);
                if let Err(e) = stdin.write_all(format!("{}\n", msg).as_bytes()).await {
                    error!("Failed to write to process stdin: {}", e);
                    break;
                }
                if let Err(e) = stdin.flush().await {
                    error!("Failed to flush process stdin: {}", e);
                    break;
                }
            }
        });

        // Reader task
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout);
            let mut line = String::new();
            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) => break, // EOF
                    Ok(_) => {
                        let trimmed = line.trim();
                        if !trimmed.is_empty() {
                            trace!("Process RX: {}", trimmed);
                            if read_tx.send(trimmed.to_string()).await.is_err() {
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to read from process stdout: {}", e);
                        break;
                    }
                }
            }
        });

        Ok(Self {
            _child: child,
            tx: write_tx,
            rx: tokio::sync::Mutex::new(read_rx),
        })
    }

    async fn send_json(&self, value: Value) -> Result<()> {
        let json = serde_json::to_string(&value)?;
        self.tx.send(json).await?;
        Ok(())
    }
}

#[async_trait]
impl Transport for ProcessTransport {
    async fn send_request(&self, request: JsonRpcRequest) -> Result<()> {
        self.send_json(serde_json::to_value(&request)?).await
    }

    async fn send_response(&self, response: JsonRpcResponse) -> Result<()> {
        self.send_json(serde_json::to_value(&response)?).await
    }

    async fn send_notification(&self, notification: JsonRpcNotification) -> Result<()> {
        self.send_json(serde_json::to_value(&notification)?).await
    }

    async fn receive(&self) -> Result<Option<McpMessage>> {
        let mut rx = self.rx.lock().await;
        match rx.recv().await {
            Some(line) => {
                let value: Value = serde_json::from_str(&line)?;
                let msg = McpMessage::from_json(value)?;
                Ok(Some(msg))
            }
            None => Ok(None),
        }
    }

    async fn close(&self) -> Result<()> {
        Ok(())
    }
}
