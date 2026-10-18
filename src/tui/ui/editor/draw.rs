//! Native editor-field projection and shared drawing entry points.
use crate::tui::app::state::{App, AppState};
pub use codetether_editor::draw::draw;
use codetether_editor::viewport::View;
use ratatui::Frame;

/// Borrow application fields needed to render an editor frame.
pub(crate) fn view(state: &mut AppState) -> View<'_> {
    View {
        buffer: state.editor.as_ref(),
        scroll: &mut state.editor_scroll,
        hscroll: &mut state.editor_hscroll,
        lsp_area: &mut state.editor_lsp.area,
        hover: state.editor_lsp.hover.as_deref(),
        goal_active: state.goal_editor.is_some(),
        status: &state.status,
    }
}

/// Draw the active document, following its cursor and showing hover text.
pub fn draw_active(f: &mut Frame, app: &mut App) {
    codetether_editor::viewport::draw_active(f, &mut view(&mut app.state));
}
