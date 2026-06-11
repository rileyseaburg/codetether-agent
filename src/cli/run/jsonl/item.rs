#![allow(dead_code)]

use super::event::RunEvent;
use anyhow::Result;

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
