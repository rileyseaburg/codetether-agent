//! MCP version negotiation and initialized notification.
use super::Client;
use anyhow::{Context, Result, ensure};
use serde_json::json;

impl Client {
    pub(crate) async fn initialize(&mut self) -> Result<()> {
        let result = self.rpc("initialize", json!({
            "protocolVersion": self.version,
            "capabilities": {}, "clientInfo": {"name": "codetether", "version": env!("CARGO_PKG_VERSION")}
        })).await?;
        let version = result["protocolVersion"]
            .as_str()
            .context("Missing MCP protocol version")?;
        ensure!(
            ["2024-11-05", "2025-03-26", "2025-06-18"].contains(&version),
            "Unsupported Monarch MCP protocol version"
        );
        self.version = version.into();
        self.post(
            json!({"jsonrpc":"2.0", "method":"notifications/initialized"}),
            None,
        )
        .await?;
        Ok(())
    }
}
