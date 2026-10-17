//! Standard I/O channel ownership and transport construction.
use crate::*;
mod protocol;
mod reader;
mod writer;

/// Stdio transport for MCP using dedicated blocking reader/writer threads.
///
/// # Examples
/// ```no_run
/// let transport = codetether_mcp_transport::StdioTransport::new();
/// ```
pub struct StdioTransport {
    /// Sender channel for outgoing messages (kept alive for transport lifetime)
    tx: mpsc::Sender<String>,
    rx: tokio::sync::Mutex<mpsc::Receiver<String>>,
}

impl Default for StdioTransport {
    fn default() -> Self {
        Self::new()
    }
}
impl StdioTransport {
    /// Create a new stdio transport
    pub fn new() -> Self {
        let (write_tx, write_rx) = mpsc::channel::<String>(100);
        let (read_tx, read_rx) = mpsc::channel::<String>(100);

        // Spawn writer thread (blocking IO)
        writer::spawn(write_rx);
        reader::spawn(read_tx);
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
