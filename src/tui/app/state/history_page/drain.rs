//! Merge completed history pages into the chronological chat buffer.

use crate::tui::app::state::App;

pub fn drain(app: &mut App) -> bool {
    let Some(result) = app.state.history_page.take_result() else {
        return false;
    };
    match result {
        Ok(page) => apply(app, page),
        Err(error) => {
            app.state.history_page.fail();
            app.state.status = format!("Failed to load older history: {error}");
        }
    }
    true
}

fn apply(app: &mut App, page: super::Page) {
    let follow_latest = app.state.chat_scroll >= crate::tui::constants::SCROLL_BOTTOM;
    let old_scroll = app.state.manual_chat_scroll();
    let old_lines = app.state.cached_message_lines.len().max(
        app.state
            .history_page
            .known_total_lines(app.state.chat_last_max_scroll),
    );
    let mut older =
        crate::tui::app::message_text::provider_messages_to_chat_messages(page.messages.iter());
    let visible_added = older.len();
    older.append(&mut app.state.messages);
    app.state.messages = older;
    crate::tui::app::message_cache_invalidate::clear(&mut app.state);
    app.state
        .history_page
        .accept(&page, old_lines, old_scroll, follow_latest);
    app.state.status = if page.exhausted {
        "Loaded the beginning of this session".to_string()
    } else {
        format!("Loaded {visible_added} older messages")
    };
}
