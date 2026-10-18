//! Borrow the chat viewport and connect older-history requests.
use super::AppState;
pub use codetether_tool_panel::TOOL_PREVIEW_FOLLOW;

/// Borrow navigation using the current pager, page size and visible message count.
pub(crate) fn view(
    state: &mut AppState,
) -> codetether_chat_layout::Navigation<'_, impl FnMut(usize) + '_> {
    let visible_messages = state.messages.len();
    let page_step = state.history_page.page_step();
    let pager = &mut state.history_page;
    codetether_chat_layout::Navigation {
        viewport: &mut state.chat_viewport,
        page_step,
        load_older: move |deficit| {
            pager.request_older(deficit, visible_messages);
        },
    }
}

#[cfg(test)]
#[path = "scroll_page_tests.rs"]
mod tests;
