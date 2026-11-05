//! One authenticated MCP HTTP exchange with sanitized failures.
use super::Client;
use anyhow::{Result, ensure};
use serde_json::Value;

impl Client {
    pub(super) async fn post(&mut self, body: Value, id: Option<u64>) -> Result<Value> {
        let mut request = self
            .http
            .post(&self.endpoint)
            .bearer_auth(&self.token)
            .header("Accept", "application/json, text/event-stream")
            .header("MCP-Protocol-Version", &self.version)
            .json(&body);
        if let Some(session) = &self.session {
            request = request.header("Mcp-Session-Id", session);
        }
        let response = request
            .send()
            .await
            .map_err(|_| anyhow::anyhow!("Monarch MCP network request failed"))?;
        ensure!(
            response.status() != reqwest::StatusCode::UNAUTHORIZED,
            "Monarch authorization expired or revoked; run codetether auth monarch login"
        );
        ensure!(
            response.status().is_success(),
            "Monarch MCP returned HTTP {} (no automatic tool-call retry)",
            response.status()
        );
        if let Some(session) = response.headers().get("Mcp-Session-Id") {
            self.session = Some(session.to_str()?.to_owned());
        }
        crate::response::read(response, id).await
    }
}
