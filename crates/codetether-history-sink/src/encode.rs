//! Encode an ordered transcript suffix as newline-delimited JSON.
use anyhow::{Context, Result};
use codetether_provider_core::Message;

/// Encode `messages[start..]` as JSONL — one line per message.
///
/// Separate from the upload path so it is unit-testable without a
/// MinIO server.
/// # Arguments
/// `messages` is the ordered transcript; `start` skips already encoded entries.
/// # Returns
/// JSONL ending in a newline per emitted message, or an empty string past the end.
/// # Errors
/// Returns a message serialization error with history-sink context.
/// # Examples
/// ```rust
/// assert!(codetether_history_sink::encode_jsonl_delta(&[], 0).unwrap().is_empty());
/// ```
pub fn encode_jsonl_delta(messages: &[Message], start: usize) -> Result<String> {
    let mut buf = String::new();
    for msg in messages.iter().skip(start) {
        let line =
            serde_json::to_string(msg).context("failed to serialize Message to JSON for sink")?;
        buf.push_str(&line);
        buf.push('\n');
    }
    Ok(buf)
}
