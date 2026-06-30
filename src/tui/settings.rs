use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::tui::app::state::AppState;
use crate::tui::status::render_status;

#[path = "settings_rows.rs"]
mod rows;

use ratatui::text::Line;
use rows::{access_mode_line, setting_line};

fn settings_lines(s: &AppState) -> Vec<Line<'static>> {
    let idx = s.selected_settings_index;
    vec![
        Line::from("Settings"),
        Line::from(""),
        setting_line("Edit auto-apply", s.auto_apply_edits, idx == 0),
        Line::from("  Automatically confirms pending edit/multiedit previews in the TUI."),
        Line::from(""),
        setting_line("Network access", s.allow_network, idx == 1),
        Line::from("  Allows sandboxed bash commands in this TUI session to use network access."),
        Line::from(""),
        setting_line("Slash autocomplete", s.slash_autocomplete, idx == 2),
        Line::from("  Enables Tab completion for slash commands in the composer."),
        Line::from(""),
        setting_line("Worktree isolation", s.use_worktree, idx == 3),
        Line::from("  Runs agent work in a git worktree branch, auto-merged on success."),
        Line::from(""),
        access_mode_line(idx == 4),
        Line::from("  Cycles tool access: ask -> approve -> full (Enter to change)."),
        Line::from(""),
        Line::from("Controls:"),
        Line::from("  - Up / Down selects a setting"),
        Line::from("  - Enter toggles or cycles the selected setting"),
        Line::from("  - Esc returns to chat"),
    ]
}

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
    use super::settings_lines;
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
