//! Authenticated HTTP SSE transport.
use super::*;
impl BusBridge {
    /// Single SSE connection attempt.  Reads until the stream closes or errors.
    pub(super) async fn read_sse_stream(&self) -> anyhow::Result<()> {
        let client = reqwest::Client::new();
        let mut req = client
            .get(&self.bus_url)
            .header("Accept", "text/event-stream");
        if let Some(token) = self
            .auth_token
            .as_deref()
            .filter(|value| !value.trim().is_empty())
        {
            req = req.bearer_auth(token);
        }
        let resp = req.send().await?;

        if !resp.status().is_success() {
            anyhow::bail!("SSE endpoint returned {}", resp.status());
        }

        self.connected.store(true, Ordering::Relaxed);
        tracing::info!(target: "codetether_agent::mcp::bus_bridge", "BusBridge: connected to SSE stream");

        // Read line-by-line.  SSE format:
        //   event: <type>\n
        //   data: <json>\n
        //   \n
        let mut event_type = String::new();
        let mut data_buf = String::new();

        use futures::StreamExt;
        let mut byte_stream = resp.bytes_stream();

        // Accumulate raw bytes into lines
        let mut line_buf = String::new();

        while let Some(chunk) = byte_stream.next().await {
            let chunk = chunk?;
            let text = String::from_utf8_lossy(&chunk);

            for ch in text.chars() {
                if ch == '\n' {
                    let line = std::mem::take(&mut line_buf);
                    self.process_sse_line(&line, &mut event_type, &mut data_buf)
                        .await;
                } else {
                    line_buf.push(ch);
                }
            }
        }

        Ok(())
    }
}
