//! Tool-call complete event.

use super::View;
use codetether_chat_model::{ChatMessage, MessageType};
use codetether_util::preview_chars::truncate_preview;

/// Apply tool-event presentation through frontend state bindings.
/// # Arguments
/// Receives frontend state and owned tool-event data.
/// # Returns
/// Applies timing, message and status effects in order.
/// # Examples
/// ```no_run
/// # async fn example(view: &mut impl codetether_chat_events::tools::View) {
/// let _ = codetether_chat_events::tools::complete(view, String::new(), String::new(), true, 0);
/// # }
/// ```
pub fn complete(
    view: &mut impl View,
    name: String,
    output: String,
    success: bool,
    duration_ms: u64,
) {
    view.reset_tool_preview_scroll();
    view.messages().push(ChatMessage::new(
        MessageType::ToolResult {
            name: name.clone(),
            output: codetether_chat_model::payload::tool_output(&output),
            success,
            duration_ms: Some(duration_ms),
        },
        format!("{name}: {}", truncate_preview(&output, 600)),
    ));
    view.note_tool_completed(name.clone(), duration_ms, success);
    view.set_status(status_text(view, &name, success, duration_ms, &output));
    view.scroll_to_bottom();
}

fn status_text(
    view: &impl View,
    name: &str,
    success: bool,
    duration_ms: u64,
    output: &str,
) -> String {
    if !success
        && let Some(status) =
            super::approval_hint::status_text(name, output, || view.approval_guidance())
    {
        return status;
    }
    if !success && let Some(status) = super::block_hint::status_text(name, output) {
        return status;
    }
    let mark = if success { "✓" } else { "✗" };
    let preview = truncate_preview(&output.replace('\n', " "), 80);
    format!("{mark} {name} finished in {duration_ms}ms — {preview}")
}
