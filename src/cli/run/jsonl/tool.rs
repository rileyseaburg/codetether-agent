#![allow(dead_code)]

use super::event::RunEvent;
use anyhow::Result;

pub(in crate::cli::run) fn write_tool_call_started(
    item_id: &str,
    tool_call_id: &str,
    timestamp_ms: u64,
) -> Result<()> {
    super::writer::write_stdout(&RunEvent::ToolCallStarted {
        item_id,
        tool_call_id,
        timestamp_ms,
        name: None,
        arguments: None,
    })
}

pub(in crate::cli::run) fn write_tool_call_completed(
    item_id: &str,
    tool_call_id: &str,
    timestamp_ms: u64,
) -> Result<()> {
    super::writer::write_stdout(&RunEvent::ToolCallCompleted {
        item_id,
        tool_call_id,
        timestamp_ms,
        name: None,
        success: None,
        duration_ms: None,
        output: None,
    })
}
