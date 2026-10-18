//! Borrow native view and viewport fields for shared transitions.
use super::AppState;

/// Borrow the current panel and all scroll fields needed for view transitions.
pub(crate) fn transition(state: &mut AppState) -> codetether_tui_navigation::ViewTransition<'_> {
    state.view.transition(
        &mut state.chat_viewport.chat_scroll,
        &mut state.chat_viewport.chat_auto_follow,
        &mut state.tool_viewport.tool_preview_scroll,
    )
}
