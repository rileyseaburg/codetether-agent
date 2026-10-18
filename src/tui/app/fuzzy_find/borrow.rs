//! Borrow frontend fields for the editor crate's finder view.
use crate::tui::{app::state::AppState, models::ViewMode};
use codetether_editor::interactive::EditorView;

pub(super) fn view(state: &mut AppState) -> EditorView<'_, impl FnMut() + '_> {
    let current = &mut state.view_mode;
    let saved = &mut state.saved_scroll;
    let chat_scroll = state.chat_viewport.chat_scroll;
    let chat_auto_follow = state.chat_viewport.chat_auto_follow;
    let tool_scroll = state.tool_viewport.tool_preview_scroll;
    EditorView {
        finder: &mut state.fuzzy_find,
        buffer: &mut state.editor,
        vertical_scroll: &mut state.editor_scroll,
        horizontal_scroll: &mut state.editor_hscroll,
        status: &mut state.status,
        needs_redraw: &mut state.needs_redraw,
        show_editor: move || {
            codetether_tui_navigation::view_state::set_view_mode(
                current,
                ViewMode::Editor,
                saved,
                chat_scroll,
                chat_auto_follow,
                tool_scroll,
            );
        },
    }
}
