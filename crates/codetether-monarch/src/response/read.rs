//! Bounded streaming HTTP-body collection and response selection.
use super::{frames, result};
use anyhow::{Result, bail, ensure};
use futures::StreamExt;
use serde_json::Value;
const MAX_BYTES: usize = 4 * 1024 * 1024;

pub(crate) async fn read(response: reqwest::Response, id: Option<u64>) -> Result<Value> {
    let Some(id) = id else {
        return Ok(Value::Null);
    };
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let sse = content_type.starts_with("text/event-stream");
    ensure!(
        sse || content_type.starts_with("application/json"),
        "Unexpected MCP content type"
    );
    let mut stream = response.bytes_stream();
    let mut bytes = Vec::new();
    let mut total = 0;
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|_| anyhow::anyhow!("MCP response stream interrupted"))?;
        total += chunk.len();
        ensure!(total <= MAX_BYTES, "MCP response exceeds size limit");
        bytes.extend_from_slice(&chunk);
        if sse {
            if let Some(value) = frames::take(&mut bytes, id)? {
                return Ok(value);
            }
        }
    }
    if !sse {
        let value = serde_json::from_slice(&bytes)
            .map_err(|_| anyhow::anyhow!("Invalid MCP response JSON"))?;
        if let Some(value) = result(value, id)? {
            return Ok(value);
        }
    }
    bail!("MCP stream ended without the requested response")
}
