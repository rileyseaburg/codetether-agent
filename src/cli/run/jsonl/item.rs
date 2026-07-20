#![allow(dead_code)]

use super::event::RunEvent;
use crate::session::thread_store::ThreadEvent;
use anyhow::Result;

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

pub(in crate::cli::run) fn write_item_started(item_id: &str, timestamp_ms: u64) -> Result<()> {
    super::writer::write_stdout(&RunEvent::ItemStarted {
        item_id,
        timestamp_ms,
    })
}

pub(in crate::cli::run) fn write_item_completed(item_id: &str, timestamp_ms: u64) -> Result<()> {
    super::writer::write_stdout(&RunEvent::ItemCompleted {
        item_id,
        timestamp_ms,
    })
}
