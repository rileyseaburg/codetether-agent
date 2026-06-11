use super::event::RunEvent;
use serde_json::Value;

mod approval;
mod approval_decision;
mod codex_thread;
mod command;
mod item;
mod lifecycle;
mod patch;
mod thread;
mod thread_tool;
mod tool;

fn event_value(event: &RunEvent<'_>) -> Value {
    serde_json::from_slice(&event_bytes(event)).unwrap()
}

fn event_bytes(event: &RunEvent<'_>) -> Vec<u8> {
    let mut out = Vec::new();
    super::writer::write_event(&mut out, event).unwrap();
    out
}
