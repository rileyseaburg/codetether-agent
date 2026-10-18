//! Goal editor viewport with persistent save/cancel help and conflict messages.

use super::{View, draw_active};
use crate::{
    draw,
    render::text_width,
    scroll::{follow_col, follow_cursor},
};
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    widgets::Paragraph,
};

/// Draw a goal draft with persistent save/discard help and status text.
/// # Arguments
/// * `frame` - Destination frame.
/// * `view` - Borrowed editor and goal fields.
/// # Returns
/// Draws the goal layout, or ordinary editor layout when no goal is active.
/// # Examples
/// ```no_run
/// fn draw(f: &mut ratatui::Frame, view: &mut codetether_editor::viewport::View<'_>) {
///     codetether_editor::viewport::draw_goal(f, view);
/// }
/// ```
pub fn draw_goal(frame: &mut Frame, view: &mut View<'_>) {
    if !view.goal_active {
        draw_active(frame, view);
        return;
    }
    let areas = Layout::vertical([Constraint::Min(3), Constraint::Length(2)]).split(frame.area());
    let area = areas[0];
    *view.lsp_area = area;
    if let Some(buffer) = view.buffer {
        *view.scroll = follow_cursor(
            buffer.backend(),
            *view.scroll,
            area.height.saturating_sub(2) as usize,
        );
        *view.hscroll = follow_col(
            buffer.backend(),
            *view.hscroll,
            text_width(buffer.backend(), area.width),
        );
        draw::draw(frame, area, buffer, *view.scroll, *view.hscroll);
    }
    frame.render_widget(
        Paragraph::new(format!(
            "Ctrl+S save goal · Esc discard · Enter newline\n{}",
            view.status
        )),
        areas[1],
    );
}
