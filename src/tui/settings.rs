use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::tui::app::state::AppState;
use crate::tui::status::render_status;

fn setting_value_span(enabled: bool) -> Span<'static> {
    if enabled {
        "ON".green().into()
    } else {
        "OFF".yellow().into()
    }
}

fn setting_line(label: &'static str, enabled: bool, selected: bool) -> Line<'static> {
    let prefix = if selected { "> " } else { "  " };
    let label_style = if selected {
        Style::default().fg(Color::Cyan).bold()
    } else {
        Style::default()
    };

    Line::from(vec![
        Span::styled(prefix, label_style),
        Span::styled(format!("{label}: "), label_style),
        setting_value_span(enabled),
    ])
}

fn settings_lines(app_state: &AppState) -> Vec<Line<'static>> {
    vec![
        Line::from("Settings"),
        Line::from(""),
        setting_line(
            "Edit auto-apply",
            app_state.auto_apply_edits,
            app_state.selected_settings_index == 0,
        ),
        Line::from("  Automatically confirms pending edit/multiedit previews in the TUI."),
        Line::from(""),
        setting_line(
            "Network access",
            app_state.allow_network,
            app_state.selected_settings_index == 1,
        ),
        Line::from("  Allows sandboxed bash commands in this TUI session to use network access."),
        Line::from(""),
        setting_line(
            "Slash autocomplete",
            app_state.slash_autocomplete,
            app_state.selected_settings_index == 2,
        ),
        Line::from("  Enables Tab completion for slash commands in the composer."),
        Line::from(""),
        setting_line(
            "Worktree isolation",
            app_state.use_worktree,
            app_state.selected_settings_index == 3,
        ),
        Line::from("  Runs agent work in a git worktree branch, auto-merged on success."),
        Line::from(""),
        Line::from("Controls:"),
        Line::from("  - Up / Down selects a setting"),
        Line::from("  - Enter toggles the selected setting"),
        Line::from("  - A toggles edit auto-apply"),
        Line::from("  - N toggles network access"),
        Line::from("  - Tab toggles slash autocomplete"),
        Line::from("  - /autoapply on|off|toggle|status works from chat"),
        Line::from("  - /settings opens this panel"),
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
        assert!(text.contains("Network access"));
        assert!(text.contains("Slash autocomplete"));
        assert!(text.contains("Up / Down selects a setting"));
        assert!(text.contains("/autoapply on|off|toggle|status"));
    }
}
