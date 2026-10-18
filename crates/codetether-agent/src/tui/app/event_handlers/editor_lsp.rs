//! Bind application fields to shared editor interaction policy.
use crate::{
    lsp::client::LspClient,
    tui::{app::state::App, models::ViewMode},
};
use codetether_editor_lsp::interactive::Controls;

pub(super) fn controls(app: &mut App) -> Controls<'_, LspClient> {
    Controls {
        in_editor: app.state.view.mode == ViewMode::Editor,
        approval_editing: app.state.approval_edit.is_some(),
        goal_editing: app.state.goal_editor.is_some(),
        editor: &mut app.state.editor.buffer,
        editor_scroll: &mut app.state.editor.scroll,
        editor_hscroll: &mut app.state.editor.hscroll,
        editor_lsp: &mut app.state.editor_lsp,
        status: &mut app.state.status,
        needs_redraw: &mut app.state.needs_redraw,
    }
}
