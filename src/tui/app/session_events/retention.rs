//! Native message/cache bindings for shared chat retention policy.
use crate::tui::app::state::App;
pub(super) fn trim(app: &mut App) {
    let state = &mut app.state;
    let cache = &mut state.message_cache;
    codetether_view_retention::chat::trim(
        &mut state.messages,
        state.history_page.expanded(),
        &mut state.chat_viewport.chat_scroll,
        crate::tui::constants::SCROLL_BOTTOM,
        || cache.clear(),
    );
}
