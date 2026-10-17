//! JSON request posting for the SSE transport.
use super::*;
impl SseTransport {
    pub(super) async fn send_json(&self, value: Value) -> Result<()> {
        let json = serde_json::to_string(&value)?;
        debug!(target: "codetether_agent::mcp::transport", "SSE TX: {}", json);

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
