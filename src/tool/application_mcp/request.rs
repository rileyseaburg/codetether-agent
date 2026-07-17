//! Authenticated JSON-RPC request execution for the application MCP tool.

use super::{ApplicationMcpTool, invocation};
use crate::tool::ToolResult;
use anyhow::{Result, anyhow};
use serde_json::{Value, json};

pub(super) async fn run(tool: &ApplicationMcpTool, input: Value) -> Result<ToolResult> {
    crate::tls::ensure_rustls_crypto_provider();
    let invocation = invocation::parse(input)?;
    let payload = json!({
        "jsonrpc": "2.0",
        "id": uuid::Uuid::new_v4().to_string(),
        "method": invocation.method,
        "params": invocation.params,
    });
    let response = tool
        .client
        .post(tool.endpoint.clone())
        .bearer_auth(&tool.token)
        .header("Accept", "application/json")
        .json(&payload)
        .send()
        .await?;
    let status = response.status();
    let body = response.text().await?;
    if !status.is_success() {
        return Ok(ToolResult::error(format!(
            "Application MCP HTTP {status}: {body}"
        )));
    }
    let envelope: Value = serde_json::from_str(&body)?;
    if let Some(error) = envelope.get("error") {
        return Ok(ToolResult::error(format!("Application MCP error: {error}")));
    }
    let result = envelope
        .get("result")
        .ok_or_else(|| anyhow!("application MCP response omitted result"))?;
    Ok(ToolResult::success(serde_json::to_string_pretty(result)?)
        .with_metadata("mcp_connection", json!(tool.name)))
}
