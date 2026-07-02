//! Settings panel rendering: composes rows and draws the widget.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::tui::app::state::AppState;
use crate::tui::status::render_status;

#[path = "settings_lines.rs"]
mod lines;
#[path = "settings_rows.rs"]
mod rows;

use lines::settings_lines;

pub fn render_settings(f: &mut Frame, area: Rect, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Length(3)])
        .split(area);

    let widget = Paragraph::new(settings_lines(app_state))
        .block(Block::default().borders(Borders::ALL).title("Settings"))
        .wrap(Wrap { trim: false });
    f.render_widget(widget, chunks[0]);
    render_status(f, chunks[1], &app_state.status);
}

#[cfg(test)]
mod tests {
    use super::lines::settings_lines;
    use crate::tui::app::state::AppState;

    #[test]
    fn settings_panel_mentions_adjustable_controls() {
        let text = settings_lines(&AppState::default())
            .into_iter()
            .map(|line| {
                line.spans
                    .into_iter()
                    .map(|span| span.content.into_owned())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        assert!(text.contains("Edit auto-apply"));
        assert!(text.contains("Access mode"));
        assert!(text.contains("Up / Down selects a setting"));
    }
}
