//! Tool-call lifecycle output.
#![allow(dead_code)]

use super::event::RunEvent;
use anyhow::Result;

/// Write a tool call started record to standard output.
/// # Arguments
/// * `item_id` - Item identity.
/// * `timestamp_ms` - Event timestamp in milliseconds.
/// * `tool_call_id` - Tool-call identity.
/// # Returns
/// Unit after the serialized record and newline are written.
/// # Errors
/// Returns JSON serialization or output I/O errors.
/// # Examples
/// ```no_run
/// # fn main() -> anyhow::Result<()> {
/// codetether_run_jsonl::write_tool_call_started("item-1", "call-1", 0)?;
/// # Ok(()) }
/// ```
pub fn write_tool_call_started(item_id: &str, tool_call_id: &str, timestamp_ms: u64) -> Result<()> {
    super::writer::write_stdout(&RunEvent::ToolCallStarted {
        item_id,
        tool_call_id,
        timestamp_ms,
        name: None,
        arguments: None,
    })
}

/// Write a tool call completed record to standard output.
/// # Arguments
/// * `item_id` - Item identity.
/// * `timestamp_ms` - Event timestamp in milliseconds.
/// * `tool_call_id` - Tool-call identity.
/// # Returns
/// Unit after the serialized record and newline are written.
/// # Errors
/// Returns JSON serialization or output I/O errors.
/// # Examples
/// ```no_run
/// # fn main() -> anyhow::Result<()> {
/// codetether_run_jsonl::write_tool_call_completed("item-1", "call-1", 1)?;
/// # Ok(()) }
/// ```
pub fn write_tool_call_completed(
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
