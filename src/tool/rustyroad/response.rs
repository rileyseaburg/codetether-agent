//! Bounded response parsing; backend diagnostics are not echoed into the TUI.

use anyhow::{Result, bail};
use serde_json::Value;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncReadExt};

const MAX_LINE_BYTES: u64 = 1_048_576;

pub(super) async fn read<R: AsyncBufRead + Unpin>(reader: &mut R, id: i64) -> Result<Value> {
    for _ in 0..32 {
        let mut line = String::new();
        let bytes = (&mut *reader)
            .take(MAX_LINE_BYTES + 1)
            .read_line(&mut line)
            .await?;
        if bytes == 0 {
            bail!("RustyRoad exited before responding");
        }
        if bytes as u64 > MAX_LINE_BYTES {
            bail!("RustyRoad response exceeded 1 MiB");
        }
        let Ok(value) = serde_json::from_str::<Value>(&line) else {
            continue;
        };
        if value.get("id").is_none() {
            continue;
        }
        if value["jsonrpc"] != "2.0" || value["id"] != id {
            bail!("RustyRoad returned an unexpected JSON-RPC response");
        }
        if let Some(error) = value.get("error").filter(|error| !error.is_null()) {
            bail!("RustyRoad JSON-RPC error: {error}");
        }
        return value
            .get("result")
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("RustyRoad response omitted result"));
    }
    bail!("RustyRoad produced too many non-response lines")
}
