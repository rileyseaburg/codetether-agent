//! Text and thinking stream events.

use crate::tui::app::state::App;
use crate::tui::app::text::truncate_preview;
use crate::tui::chat::message::{ChatMessage, MessageType};

pub(super) fn chunk(app: &mut App, chunk: String) {
    if app.state.chat_auto_follow {
        app.state.scroll_to_bottom();
    }
    app.state.note_text_token();
    let chunk_len = chunk.len();
    app.state
        .replace_streaming_text(super::text_bound::bounded_chunk(chunk));
    app.state.record_streaming_chars(chunk_len);
    app.state.status = format!("Streaming reply… {} chars", app.state.streaming_text.len());
}

pub(super) fn complete(app: &mut App, text: String) {
    app.state.note_text_token();
    app.state.clear_streaming_text();
    app.state
        .messages
        .push(ChatMessage::new(MessageType::Assistant, text));
    app.state.status = "Assistant replied".to_string();
    if app.state.chat_auto_follow {
        app.state.scroll_to_bottom();
    }
}

pub(super) fn thinking_complete(app: &mut App, text: String) {
    if text.is_empty() {
        return;
    }
    app.state.reset_tool_preview_scroll();
    app.state.status = format!(
        "Reasoning: {}",
        truncate_preview(&text.replace('\n', " "), 96)
    );
    app.state.messages.push(ChatMessage::new(
        MessageType::Thinking(text.clone()),
        truncate_preview(&text, 600),
    ));
    if app.state.chat_auto_follow {
        app.state.scroll_to_bottom();
    }
}
