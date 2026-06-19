//! Pure formatting helpers for S3 training-record serialization.
//!
//! Key construction, bus-message tag extraction, and `Part` flattening —
//! split out of `s3_sink.rs` to keep that module within the line budget.

use super::BusMessage;
use crate::a2a::types::Part;
use chrono::Utc;

/// Build a date-partitioned, unique S3 object key for a JSONL batch.
pub(super) fn build_s3_key(prefix: &str, now: chrono::DateTime<Utc>) -> String {
    let prefix = if prefix.is_empty() {
        String::new()
    } else if prefix.ends_with('/') {
        prefix.to_string()
    } else {
        format!("{prefix}/")
    };
    let date_path = now.format("%Y/%m/%d/%H").to_string();
    let timestamp = now.format("%Y%m%dT%H%M%S").to_string();
    let uuid = uuid::Uuid::new_v4();
    format!("{prefix}v2/{date_path}/batch_{timestamp}_{uuid}.jsonl")
}

/// Extract the serde tag name from a `BusMessage` variant.
pub(super) fn bus_message_kind(msg: &BusMessage) -> String {
    serde_json::to_value(msg)
        .ok()
        .and_then(|v| v.get("kind").and_then(|k| k.as_str()).map(String::from))
        .unwrap_or_else(|| "unknown".into())
}

/// Concatenate `Part` items into a single text string.
pub(super) fn parts_to_text(parts: &[Part]) -> String {
    parts
        .iter()
        .map(|p| match p {
            Part::Text { text } => text.as_str(),
            Part::Data { .. } => "<<data>>",
            Part::File { .. } => "<<file>>",
        })
        .collect::<Vec<_>>()
        .join("\n")
}
