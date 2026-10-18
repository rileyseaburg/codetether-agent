//! Borrow frontend fields for the approval editor crate's host.
use crate::tui::{app::state::AppState, models::ViewMode};
use codetether_approval_edit::interactive::BorrowedHost;

pub(super) fn host(state: &mut AppState) -> BorrowedHost<'_, impl FnMut() + '_> {
    let current = &mut state.view_mode;
    let saved = &mut state.saved_scroll;
    let chat_scroll = state.chat_viewport.chat_scroll;
    let chat_auto_follow = state.chat_viewport.chat_auto_follow;
    let tool_scroll = state.tool_viewport.tool_preview_scroll;
    BorrowedHost {
        editor: &mut state.editor.buffer,
        session: &mut state.approval_edit,
        vertical_scroll: &mut state.editor.scroll,
        horizontal_scroll: &mut state.editor.hscroll,
        preview_scroll: &mut state.approval_preview_scroll,
        waiting: &mut state.approval_waiting,
        status: &mut state.status,
        needs_redraw: &mut state.needs_redraw,
        show_chat: move || {
            codetether_tui_navigation::view_state::set_view_mode(
                current,
                ViewMode::Chat,
                saved,
                chat_scroll,
                chat_auto_follow,
                tool_scroll,
            );
        },
    }
}
