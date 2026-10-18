//! Durable thread event dispatch.
#![allow(dead_code)]

#[path = "codex_thread.rs"]
pub(super) mod codex_thread;

use std::io::Write;

use anyhow::Result;

use codetether_thread_store::ThreadEvent;

use super::event::RunEvent;

/// Write a supported thread event in the run JSONL format.
/// # Arguments
/// * `writer` - Destination for one JSON line.
/// * `event` - Durable event to convert.
/// # Returns
/// True after writing; false for unsupported or incomplete events.
/// # Errors
/// Returns JSON serialization or output I/O errors.
/// # Examples
/// ```
/// # fn main() -> anyhow::Result<()> {
/// use codetether_thread_store::ThreadEvent;
/// let event = ThreadEvent {
///     event_id: "e1".into(), thread_id: "t1".into(), session_id: "s1".into(),
///     turn_id: "turn1".into(), kind: "item.started".into(), timestamp_ms: 0,
///     payload: serde_json::json!({"item_id": "i1"}),
/// };
/// let mut output = Vec::new();
/// assert!(codetether_run_jsonl::write_thread_event_to(&mut output, &event)?);
/// assert!(output.ends_with(b"\n"));
/// # Ok(()) }
/// ```
pub fn write_thread_event_to<W: Write>(writer: W, event: &ThreadEvent) -> Result<bool> {
    let Some(run_event) = to_run_event(event) else {
        return Ok(false);
    };
    super::writer::write_event(writer, &run_event)?;
    Ok(true)
}

fn to_run_event(event: &ThreadEvent) -> Option<RunEvent<'_>> {
    match event.kind.as_str() {
        "item.started" | "item.delta" | "item.completed" => super::item::from_thread(event),
        "tool.started" | "tool.completed" | "tool.metadata" => {
            super::thread_tool::to_run_event(event)
        }
        "approval.requested" | "approval.decided" => super::thread_approval::to_run_event(event),
        "command.started" | "command.completed" => super::thread_command::to_run_event(event),
        "patch.started" | "patch.completed" | "patch.approval_required" => {
            super::thread_patch::to_run_event(event)
        }
        _ => None,
    }
}

pub(super) fn field<'a>(event: &'a ThreadEvent, key: &str) -> Option<&'a str> {
    event.payload.get(key)?.as_str()
}
