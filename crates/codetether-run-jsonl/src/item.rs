//! Item lifecycle and text event conversion.
#![allow(dead_code)]

use super::event::RunEvent;
use anyhow::Result;
use codetether_thread_store::ThreadEvent;

pub(super) fn from_thread(event: &ThreadEvent) -> Option<RunEvent<'_>> {
    let item_id = super::thread::field(event, "item_id")?;
    match event.kind.as_str() {
        "item.started" => Some(RunEvent::ItemStarted {
            item_id,
            timestamp_ms: event.timestamp_ms,
        }),
        "item.delta" => Some(RunEvent::TextChunk {
            item_id,
            timestamp_ms: event.timestamp_ms,
            text: super::thread::field(event, "text")?,
        }),
        "item.completed" if event.payload.get("text").is_some() => Some(RunEvent::TextComplete {
            item_id,
            timestamp_ms: event.timestamp_ms,
            text: super::thread::field(event, "text")?,
        }),
        "item.completed" => Some(RunEvent::ItemCompleted {
            item_id,
            timestamp_ms: event.timestamp_ms,
        }),
        _ => None,
    }
}

/// Write a item started record to standard output.
/// # Arguments
/// * `item_id` - Item identity.
/// * `timestamp_ms` - Event timestamp in milliseconds.
/// # Returns
/// Unit after the serialized record and newline are written.
/// # Errors
/// Returns JSON serialization or output I/O errors.
/// # Examples
/// ```no_run
/// # fn main() -> anyhow::Result<()> {
/// codetether_run_jsonl::write_item_started("item-1", 0)?;
/// # Ok(()) }
/// ```
pub fn write_item_started(item_id: &str, timestamp_ms: u64) -> Result<()> {
    super::writer::write_stdout(&RunEvent::ItemStarted {
        item_id,
        timestamp_ms,
    })
}

/// Write a item completed record to standard output.
/// # Arguments
/// * `item_id` - Item identity.
/// * `timestamp_ms` - Event timestamp in milliseconds.
/// # Returns
/// Unit after the serialized record and newline are written.
/// # Errors
/// Returns JSON serialization or output I/O errors.
/// # Examples
/// ```no_run
/// # fn main() -> anyhow::Result<()> {
/// codetether_run_jsonl::write_item_completed("item-1", 1)?;
/// # Ok(()) }
/// ```
pub fn write_item_completed(item_id: &str, timestamp_ms: u64) -> Result<()> {
    super::writer::write_stdout(&RunEvent::ItemCompleted {
        item_id,
        timestamp_ms,
    })
}
