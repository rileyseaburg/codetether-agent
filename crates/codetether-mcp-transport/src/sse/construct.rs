//! SSE transport construction, retaining the existing receive placeholder.
use super::*;
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
            debug!(target: "codetether_agent::mcp::transport", "SSE transport connecting to: {}", endpoint_clone);
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
}
