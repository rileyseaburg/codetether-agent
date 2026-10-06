//! Goal editor viewport with persistent save/cancel help and conflict messages.

use super::editor::{
    draw,
    render::text_width,
    scroll::{follow_col, follow_cursor},
};
use crate::tui::app::state::App;
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    widgets::Paragraph,
};

pub(super) fn render(frame: &mut Frame, app: &mut App) {
    if app.state.goal_editor.is_none() {
        draw::draw_active(frame, app);
        return;
    }
    let areas = Layout::vertical([Constraint::Min(3), Constraint::Length(2)]).split(frame.area());
    let area = areas[0];
    app.state.editor_lsp.area = area;
    if let Some(buffer) = app.state.editor.as_ref() {
        app.state.editor_scroll = follow_cursor(
            buffer.backend(),
            app.state.editor_scroll,
            area.height.saturating_sub(2) as usize,
        );
        app.state.editor_hscroll = follow_col(
            buffer.backend(),
            app.state.editor_hscroll,
            text_width(buffer.backend(), area.width),
        );
        draw::draw(
            frame,
            area,
            buffer,
            app.state.editor_scroll,
            app.state.editor_hscroll,
        );
    }
    frame.render_widget(
        Paragraph::new(format!(
            "Ctrl+S save goal · Esc discard · Enter newline\n{}",
            app.state.status
        )),
        areas[1],
    );
}

#[cfg(test)]
#[path = "goal_editor_tests.rs"]
mod tests;
