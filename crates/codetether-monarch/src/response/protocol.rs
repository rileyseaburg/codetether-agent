//! JSON-RPC identity validation and SSE data-field decoding.
use anyhow::{Result, ensure};
use serde_json::Value;

pub(crate) fn result(value: Value, id: u64) -> Result<Option<Value>> {
    if value.get("id").is_none() {
        return Ok(None);
    }
    ensure!(
        value["jsonrpc"] == "2.0" && value["id"] == id,
        "Unexpected MCP response identity"
    );
    ensure!(
        value.get("error").is_none(),
        "Monarch MCP returned a JSON-RPC error"
    );
    Ok(Some(
        value
            .get("result")
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Missing MCP result"))?,
    ))
}

pub(crate) fn event(frame: &str, id: u64) -> Result<Option<Value>> {
    let data = frame
        .lines()
        .filter_map(|line| line.strip_prefix("data:"))
        .map(|line| line.strip_prefix(' ').unwrap_or(line))
        .collect::<Vec<_>>()
        .join("\n");
    if data.is_empty() {
        return Ok(None);
    }
    result(
        serde_json::from_str(&data).map_err(|_| anyhow::anyhow!("Invalid MCP event JSON"))?,
        id,
    )
}
