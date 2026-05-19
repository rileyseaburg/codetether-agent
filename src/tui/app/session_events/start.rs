//! Tool-call start event.

use crate::tui::app::state::App;
use crate::tui::app::text::truncate_preview;
use crate::tui::app::worker_bridge::handle_processing_started;
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::worker_bridge::TuiWorkerBridge;

pub(crate) async fn start(
    app: &mut App,
    worker_bridge: &Option<TuiWorkerBridge>,
    name: String,
    arguments: String,
) {
    handle_processing_started(app, worker_bridge).await;
    ensure_timing(app);
    flush_streaming(app);
    app.state.reset_tool_preview_scroll();
    app.state.start_pending_tool(name.clone());
    app.state.status = format!("Running tool: {name}");
    app.state.messages.push(ChatMessage::new(
        MessageType::ToolCall {
            name: name.clone(),
            arguments: crate::tui::chat::payload::tool_arguments(&arguments),
        },
        format!("{name}: {}", truncate_preview(&arguments, 240)),
    ));
    app.state.scroll_to_bottom();
}

fn ensure_timing(app: &mut App) {
    if app.state.processing_started_at.is_none() {
        app.state.begin_request_timing();
    }
    if app.state.streaming_start.is_none() {
        app.state.begin_streaming();
    }
}

fn flush_streaming(app: &mut App) {
    if app.state.streaming_text.is_empty() {
        return;
    }
    let text = std::mem::take(&mut app.state.streaming_text);
    app.state
        .messages
        .push(ChatMessage::new(MessageType::Assistant, text));
}
