//! Application adapter for editor drawing and LSP hover display.
use super::{
    render::text_width,
    scroll::{follow_col, follow_cursor},
};
pub use codetether_editor::draw::draw;
use ratatui::Frame;

/// Draws the editor from app state if a buffer is active; no-op otherwise.
///
/// Updates `editor_scroll`/`editor_hscroll` so the cursor stays visible.
pub fn draw_active(f: &mut Frame, app: &mut crate::tui::app::state::App) {
    let area = f.area();
    app.state.editor_lsp.area = area;
    let height = area.height.saturating_sub(2) as usize;
    if let Some(buf) = app.state.editor.as_ref() {
        let w = text_width(buf.backend(), area.width);
        app.state.editor_scroll = follow_cursor(buf.backend(), app.state.editor_scroll, height);
        app.state.editor_hscroll = follow_col(buf.backend(), app.state.editor_hscroll, w);
    }
    if let Some(buf) = app.state.editor.as_ref() {
        draw(
            f,
            area,
            buf,
            app.state.editor_scroll,
            app.state.editor_hscroll,
        );
    }
    if let Some(contents) = app.state.editor_lsp.hover.as_ref() {
        super::hover_popup::draw_hover(f, area, contents);
    }
}
