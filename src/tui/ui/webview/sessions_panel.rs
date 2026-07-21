use ratatui::{prelude::*, widgets::*};

use crate::tui::app::state::App;
use crate::tui::ui::sessions::window::bounds;

#[path = "sessions_panel/rows.rs"]
mod rows;

pub fn render(f: &mut Frame, app: &App, area: Rect) {
    let mut lines = Vec::new();
    let visible = area.height.saturating_sub(2) as usize / 2;
    let selected = app
        .state
        .selected_session
        .min(app.state.sessions.len().saturating_sub(1));
    let range = bounds(selected, app.state.sessions.len(), visible);
    for idx in range {
        let summary = &app.state.sessions[idx];
        lines.push(rows::title(idx, selected, summary));
        lines.push(Line::styled(
            rows::metadata(summary),
            Style::default().fg(Color::DarkGray),
        ));
    }
    if lines.is_empty() {
        lines.push(Line::styled(
            "No recent sessions",
            Style::default().fg(Color::DarkGray),
        ));
    }
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Recent Sessions ");
    f.render_widget(Paragraph::new(lines).block(block), area);
}
