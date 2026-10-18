//! Apply shared editor buffer navigation to application fields.
use crate::{lsp::LocationInfo, tui::app::state::App};
pub(super) fn navigate(app: &mut App, loc: &LocationInfo) {
    if let Some(status) = codetether_editor_lsp::navigate(
        &mut app.state.editor,
        &mut app.state.editor_scroll,
        &mut app.state.editor_hscroll,
        loc,
    ) {
        app.state.status = status;
    }
}
