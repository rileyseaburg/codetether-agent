//! Borrow frontend fields and snapshot measurements for a single pager update.
use crate::tui::app::state::AppState;
type Source = codetether_history_page::store::Source<crate::session::Session>;

pub(super) fn borrow(
    state: &mut AppState,
) -> codetether_history_page::interactive::UpdateView<'_, Source, impl FnMut() + '_> {
    let follow_latest = state.chat_viewport.chat_scroll >= crate::tui::constants::SCROLL_BOTTOM;
    let manual_scroll = state.chat_viewport.manual_chat_scroll();
    let cached_lines = state.message_cache.cached_message_lines.len();
    let last_max_scroll = state.chat_viewport.chat_last_max_scroll;
    let cache = &mut state.message_cache;
    codetether_history_page::interactive::UpdateView {
        pager: &mut state.history_page,
        follow_latest,
        manual_scroll,
        cached_lines,
        last_max_scroll,
        messages: &mut state.messages,
        clear_cache: move || cache.clear(),
        status: &mut state.status,
        scroll: &mut state.chat_viewport.chat_scroll,
        auto_follow: &mut state.chat_viewport.chat_auto_follow,
    }
}
