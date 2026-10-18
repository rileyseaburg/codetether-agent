//! Tool-start timing, text flush and message presentation.
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
/// let _ = codetether_chat_events::tools::start(view, String::new(), String::new()).await;
/// # }
/// ```
pub async fn start(view: &mut impl View, name: String, arguments: String) {
    view.processing_started().await;
    ensure_timing(view);
    flush_streaming(view);
    let preview = truncate_preview(&arguments.replace('\n', " "), 96);
    view.reset_tool_preview_scroll();
    view.start_pending_tool(name.clone());
    view.set_status(format!("Running {name}: {preview}"));
    view.messages().push(ChatMessage::new(
        MessageType::ToolCall {
            name: name.clone(),
            arguments: codetether_chat_model::payload::tool_arguments(&arguments),
        },
        format!("{name}: {}", truncate_preview(&arguments, 240)),
    ));
    view.scroll_to_bottom();
}

fn ensure_timing(view: &mut impl View) {
    if !view.request_timing_started() {
        view.begin_request_timing();
    }
    if !view.streaming_started() {
        view.begin_streaming();
    }
}

fn flush_streaming(view: &mut impl View) {
    if view.streaming_text().is_empty() {
        return;
    }
    let text = std::mem::take(view.streaming_text());
    view.messages()
        .push(ChatMessage::new(MessageType::Assistant, text));
}
