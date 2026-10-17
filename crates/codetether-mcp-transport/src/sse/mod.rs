//! HTTP-based SSE transport state.
use crate::*;
mod construct;
mod protocol;
mod send;

/// HTTP-backed MCP transport with the existing placeholder receive stream.
///
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let transport = codetether_mcp_transport::SseTransport::new("http://localhost:8080".into()).await?;
/// # Ok(()) }
/// ```
pub struct SseTransport {
    endpoint: String,
    client: reqwest::Client,
    _tx: mpsc::Sender<String>,
    rx: tokio::sync::Mutex<mpsc::Receiver<String>>,
}
