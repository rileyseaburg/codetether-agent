//! request responsibility for the stdio LSP transport.

use super::*;

impl LspTransport {
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
}
