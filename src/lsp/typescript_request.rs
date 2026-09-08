//! Request and decode one authoritative synchronous TypeScript diagnostic response.

use super::{COMMAND, LspClient, diagnostic::Diagnostic};
use anyhow::{Context, Result, ensure};
use serde_json::json;

pub(super) async fn run(client: &LspClient, uri: &str, method: &str) -> Result<Vec<Diagnostic>> {
    let reply = client
        .transport
        .request(
            "workspace/executeCommand",
            Some(json!({
                "command": COMMAND,
                "arguments": [method, {"file": uri}, {"expectsResult": true, "isAsync": false}]
            })),
        )
        .await?;
    if let Some(error) = reply.error {
        anyhow::bail!("TypeScript {method} failed: {}", error.message);
    }
    let result = reply
        .result
        .context("TypeScript diagnostic response missing result")?;
    ensure!(
        result["success"] == true,
        "TypeScript {method} failed: {result}"
    );
    let body = result
        .get("body")
        .cloned()
        .context("TypeScript diagnostics missing body")?;
    serde_json::from_value(body).context("Invalid TypeScript diagnostic response")
}
