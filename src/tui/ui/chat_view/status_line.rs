//! Bottom status-line renderer widget.

use ratatui::{text::Line, widgets::Paragraph, Frame};

use super::status::build_status_spans;
use crate::tui::app::state::App;

/// Render the bottom status line with keybinding hints and badges.
///
/// # Examples
///
/// ```rust,no_run
/// # use codetether_agent::tui::ui::chat_view::status_line::render_status_line;
/// # fn d(f:&mut ratatui::Frame,a:&codetether_agent::tui::app::state::App){ render_status_line(f,a,ratatui::layout::Rect::new(0,23,80,1)); }
/// ```
pub fn render_status_line(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let session_label = app
        .state
        .session_id
        .as_deref()
        .map(|id| id.to_string())
        .unwrap_or_else(|| "new".to_string());
    let spans = build_status_spans(app, &session_label);
    f.render_widget(Paragraph::new(Line::from(spans)), area);
}
