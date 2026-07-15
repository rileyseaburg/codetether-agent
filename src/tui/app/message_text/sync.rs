//! Synchronize canonical provider messages into the live chat viewport.

use crate::provider::Message;
use crate::session::Session;
use crate::tui::app::state::App;

pub fn sync_messages_from_session(app: &mut App, session: &Session) {
    sync_messages_from_source(app, session, &session.id, false);
}

pub(crate) fn sync_messages_from_source(
    app: &mut App,
    session: &Session,
    source_id: &str,
    source_has_older: bool,
) {
    let visible: Vec<_> = crate::tui::app::message_window::recent(session).collect();
    let boundary: Vec<Message> = visible.iter().take(3).map(|item| (*item).clone()).collect();
    let has_older = source_has_older || session.history().len() > visible.len();
    app.state.messages = super::provider_messages_to_chat_messages(visible.iter().copied());
    app.state
        .history_page
        .reset(source_id.to_string(), &boundary, visible.len(), has_older);
    crate::tui::app::message_cache_invalidate::clear(&mut app.state);
    reset_metrics(app);
    app.state.reset_tool_preview_scroll();
    app.state.set_tool_preview_max_scroll(0);
    app.state.scroll_to_bottom();
}

fn reset_metrics(app: &mut App) {
    app.state.current_request_first_token_ms = None;
    app.state.current_request_last_token_ms = None;
    app.state.last_request_first_token_ms = None;
    app.state.last_request_last_token_ms = None;
    app.state.last_completion_model = None;
    app.state.last_completion_latency_ms = None;
    app.state.last_completion_prompt_tokens = None;
    app.state.last_completion_output_tokens = None;
    app.state.last_tool_name = None;
    app.state.last_tool_latency_ms = None;
    app.state.last_tool_success = None;
    app.state.chat_latency.clear();
}
