//! Bottom status-line renderer widget.

use ratatui::{Frame, widgets::Paragraph};

use super::status::build_status_lines;
use crate::tui::app::state::App;

/// Render the bottom status line with keybinding hints and badges.
///
/// Stacks across multiple rows when the terminal is narrower than
/// [`super::status::STACK_WIDTH_THRESHOLD`].
///
/// # Examples
///
/// ```rust,no_run
/// # use codetether_agent::tui::ui::chat_view::status_line::render_status_line;
/// # fn d(f:&mut ratatui::Frame,a:&codetether_agent::tui::app::state::App){ render_status_line(f,a,ratatui::layout::Rect::new(0,23,80,3)); }
/// ```
pub fn render_status_line(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let session_label = app
        .state
        .session_id
        .as_deref()
        .map(|id| id.to_string())
        .unwrap_or_else(|| "new".to_string());
    let lines = build_status_lines(app, &session_label, area.width);
    f.render_widget(Paragraph::new(lines), area);
}
