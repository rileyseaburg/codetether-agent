//! Result delivery and transcript presentation.
use crate::{ShellEvent, View};
use codetether_chat_model::{ChatMessage, MessageType};
/// Apply the latest available result and clear the in-flight state.
/// # Arguments
/// Receives the frontend that owns the result channel and transcript.
/// # Returns
/// True if a result was applied and a redraw is needed.
pub fn drain(view: &mut impl View) -> bool {
    let Some(event) = view.shell().take_latest() else {
        return false;
    };
    apply(view, event);
    true
}
fn apply(view: &mut impl View, event: ShellEvent) {
    view.push_message(ChatMessage::new(
        MessageType::ToolResult {
            name: "shell".to_string(),
            output: event.output.clone(),
            success: event.success,
            duration_ms: Some(event.duration_ms),
        },
        event.output,
    ));
    view.set_status(if event.success {
        format!("Shell command finished ({} ms)", event.duration_ms)
    } else {
        format!("Shell command failed ({} ms)", event.duration_ms)
    });
    view.scroll_to_bottom();
}
