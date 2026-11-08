//! Incremental SSE frame boundaries and buffered event extraction.
use super::event;
use anyhow::Result;
use serde_json::Value;

pub(super) fn frame_end(bytes: &[u8]) -> Option<usize> {
    (1..bytes.len()).find_map(|i| {
        if bytes[i - 1..=i] == *b"\n\n" || (i >= 3 && bytes[i - 3..=i] == *b"\r\n\r\n") {
            Some(i + 1)
        } else {
            None
        }
    })
}

pub(super) fn take(bytes: &mut Vec<u8>, id: u64) -> Result<Option<Value>> {
    while let Some(end) = frame_end(bytes) {
        let frame = std::str::from_utf8(&bytes[..end])
            .map_err(|_| anyhow::anyhow!("Invalid MCP event encoding"))?;
        if let Some(value) = event(frame, id)? {
            return Ok(Some(value));
        }
        bytes.drain(..end);
    }
    Ok(None)
}
