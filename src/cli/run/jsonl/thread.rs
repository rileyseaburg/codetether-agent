#![allow(dead_code)]

#[path = "codex_thread.rs"]
pub(super) mod codex_thread;

use std::io::Write;

use anyhow::Result;

use crate::session::thread_store::ThreadEvent;

use super::event::RunEvent;

pub(in crate::cli::run) fn write_thread_event_to<W: Write>(
    writer: W,
    event: &ThreadEvent,
) -> Result<bool> {
    let Some(run_event) = to_run_event(event) else {
        return Ok(false);
    };
    super::writer::write_event(writer, &run_event)?;
    Ok(true)
}

fn to_run_event(event: &ThreadEvent) -> Option<RunEvent<'_>> {
    match event.kind.as_str() {
        "item.started" => Some(RunEvent::ItemStarted {
            item_id: field(event, "item_id")?,
            timestamp_ms: event.timestamp_ms,
        }),
        "item.completed" => Some(RunEvent::ItemCompleted {
            item_id: field(event, "item_id")?,
            timestamp_ms: event.timestamp_ms,
        }),
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
