//! Drain background `!command` results into the chat transcript.

use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};

use super::event::ShellEvent;

/// Pull any finished `!command` results from `shell_rx` and append them to the
/// transcript. Called from the event loop each tick. Returns `true` if the UI
/// changed and a redraw is needed.
pub(crate) fn drain_shell_events(app: &mut App) -> bool {
    let mut latest: Option<ShellEvent> = None;
    if let Some(rx) = app.state.shell_rx.as_mut() {
        while let Ok(event) = rx.try_recv() {
            latest = Some(event);
        }
    }
    let Some(event) = latest else {
        return false;
    };
    app.state.shell_rx = None;
    app.state.shell_running = false;
    apply(app, event);
    true
}

fn apply(app: &mut App, event: ShellEvent) {
    app.state.messages.push(ChatMessage::new(
        MessageType::ToolResult {
            name: "shell".to_string(),
            output: event.output.clone(),
            success: event.success,
            duration_ms: Some(event.duration_ms),
        },
        event.output,
    ));
    app.state.status = if event.success {
        format!("Shell command finished ({} ms)", event.duration_ms)
    } else {
        format!("Shell command failed ({} ms)", event.duration_ms)
    };
    app.state.scroll_to_bottom();
}
