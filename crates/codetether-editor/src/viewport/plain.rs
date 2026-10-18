//! Ordinary editor drawing, cursor following and hover display.
use super::View;
use crate::draw::draw;
use crate::{
    render::text_width,
    scroll::{follow_col, follow_cursor},
};
use ratatui::Frame;

/// Draw the active document, followed by any hover popup.
///
/// Updates both scroll offsets so the cursor stays visible.
/// # Arguments
/// * `f` - Destination frame.
/// * `view` - Borrowed editor fields and mutable scroll/rectangle state.
/// # Returns
/// Draws the open document and hover popup, updating the viewport offsets.
/// # Examples
/// ```no_run
/// fn draw(f: &mut ratatui::Frame, view: &mut codetether_editor::viewport::View<'_>) {
///     codetether_editor::viewport::draw_active(f, view);
/// }
/// ```
pub fn draw_active(f: &mut Frame, view: &mut View<'_>) {
    let area = f.area();
    *view.lsp_area = area;
    let height = area.height.saturating_sub(2) as usize;
    if let Some(buf) = view.buffer {
        let w = text_width(buf.backend(), area.width);
        *view.scroll = follow_cursor(buf.backend(), *view.scroll, height);
        *view.hscroll = follow_col(buf.backend(), *view.hscroll, w);
    }
    if let Some(buf) = view.buffer {
        draw(f, area, buf, *view.scroll, *view.hscroll);
    }
    if let Some(contents) = view.hover {
        crate::hover_popup::draw_hover(f, area, contents);
    }
}
