use super::length::{batch_bounds, trim_start};
use anyhow::{Result, anyhow, ensure};
use serde_json::Value;

// Each length includes the leading newline after the marker and trailing newline
// after the JSON. Operate on bytes, not Unicode scalar or UTF-16 character counts.
pub(super) fn decode(body: &[u8]) -> Result<Vec<Value>> {
    ensure!(body.len() >= 5, "truncated_rpc_stream");
    ensure!(body.starts_with(b")]}'"), "missing_anti_xssi_prefix");
    let prefix = body
        .iter()
        .position(|b| *b == b'\n')
        .ok_or_else(|| anyhow!("truncated_rpc_stream"))?;
    let mut remaining = &body[prefix + 1..];
    let mut frames = Vec::new();
    loop {
        remaining = trim_start(remaining);
        if remaining.is_empty() {
            return Ok(frames);
        }
        let (start, end) = batch_bounds(remaining)?;
        let batch: Value = serde_json::from_slice(&remaining[start..end])
            .map_err(|error| anyhow::Error::new(error).context("invalid_batch_json"))?;
        let Value::Array(batch) = batch else {
            return Err(anyhow!("invalid_batch_shape"));
        };
        ensure!(
            batch
                .iter()
                .all(|frame| frame.is_array() && frame.get(0).is_some_and(Value::is_string)),
            "invalid_batch_shape"
        );
        frames.extend(batch);
        remaining = &remaining[end..];
    }
}
