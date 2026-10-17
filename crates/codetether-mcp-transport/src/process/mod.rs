//! Owned subprocess transport and outgoing JSON encoding.
use crate::*;
mod construct;
mod protocol;
mod reader;
mod stderr;
mod writer;

/// Process transport for connecting to MCP servers via subprocess.
///
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let transport = codetether_mcp_transport::ProcessTransport::spawn("mcp-server", &[]).await?;
/// # Ok(()) }
/// ```
pub struct ProcessTransport {
    _child: tokio::process::Child,
    tx: mpsc::Sender<String>,
    rx: tokio::sync::Mutex<mpsc::Receiver<String>>,
}
impl ProcessTransport {
    async fn send_json(&self, value: Value) -> Result<()> {
        let json = serde_json::to_string(&value)?;
        self.tx.send(json).await?;
        Ok(())
    }
}
