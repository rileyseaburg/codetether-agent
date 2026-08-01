//! Task-scoped sink for structured session events.
//!
//! The tool-execution path is reached through several layers whose callbacks are
//! all `Fn(String)`. Widening every signature to carry structure would touch the
//! swarm, session, and response modules for one leaf concern, so the active
//! task's event sink is published here instead and read at the emit site.
//!
//! When no sink is installed every helper degrades to a no-op, which keeps
//! non-worker callers (the TUI, `codetether run`) unchanged.

use std::sync::{Arc, RwLock};

use super::task_output::EventSink;

static ACTIVE_SINK: RwLock<Option<EventSink>> = RwLock::new(None);

/// Installs the sink used for the currently executing task.
///
/// # Examples
///
/// ```ignore
/// install_sink(Some(sink));
/// ```
pub(super) fn install_sink(sink: Option<EventSink>) {
    if let Ok(mut slot) = ACTIVE_SINK.write() {
        *slot = sink;
    }
}

/// Returns the installed sink, if any.
fn sink() -> Option<EventSink> {
    ACTIVE_SINK.read().ok().and_then(|slot| slot.clone())
}

/// Emits one typed event with its human-readable text.
///
/// # Examples
///
/// ```ignore
/// emit("[tool:start:bash]".into(), tool_call_event("bash", &args));
/// ```
pub(super) fn emit(text: String, event: serde_json::Value) {
    if let Some(sink) = sink() {
        sink(text, Some(event));
    }
}

/// Emits a `tool.call` event and reports whether a sink consumed it.
///
/// Callers use the return value to decide whether the legacy text-only callback
/// still needs to fire, so non-worker contexts keep their existing output.
pub(super) fn emit_tool_call_if_active(tool_name: &str, arguments: &serde_json::Value) -> bool {
    if sink().is_none() {
        return false;
    }
    emit_tool_call(tool_name, arguments);
    true
}

/// Emits a `tool.result` event and reports whether a sink consumed it.
pub(super) fn emit_tool_result_if_active(
    tool_name: &str,
    success: bool,
    output: &str,
    duration_ms: Option<u128>,
) -> bool {
    if sink().is_none() {
        return false;
    }
    emit_tool_result(tool_name, success, output, duration_ms);
    true
}

/// Emits a `tool.call` event for one invocation.
pub(super) fn emit_tool_call(tool_name: &str, arguments: &serde_json::Value) {
    emit(
        format!("[tool:start:{tool_name}]"),
        super::session_event::tool_call_event(tool_name, arguments),
    );
}

/// Emits a `tool.result` event, preserving the legacy text form.
///
/// The text keeps the historical `[tool:name:ok] …` shape so existing log
/// readers and the plain output stream are unaffected.
pub(super) fn emit_tool_result(
    tool_name: &str,
    success: bool,
    output: &str,
    duration_ms: Option<u128>,
) {
    let status = if success { "ok" } else { "err" };
    let preview = crate::util::truncate_bytes_safe(output, 500);
    emit(
        format!("[tool:{tool_name}:{status}] {preview}"),
        super::session_event::tool_result_event(tool_name, success, output, duration_ms),
    );
}

/// Emits an `agent.reasoning` event when the provider exposed reasoning text.
pub(super) fn emit_reasoning(text: &str) {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return;
    }
    emit(
        String::new(),
        super::session_event::reasoning_event(trimmed),
    );
}

#[cfg(test)]
#[path = "session_event_sink_tests.rs"]
mod tests;
