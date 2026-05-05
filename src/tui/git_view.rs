//! Render the `/git` TUI view — branch, log, diff, branches.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::tui::app::state::git_state::GitViewState;
use crate::tui::input::render_input_preview;

/// Render the git status view into `area` of `f`.
pub fn render_git_view(f: &mut Frame, area: Rect, state: &GitViewState, status: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Length(3)])
        .split(area);

    let lines = build_git_lines(state);
    let widget = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Git"))
        .wrap(Wrap { trim: false });
    f.render_widget(widget, chunks[0]);
    render_input_preview(f, chunks[1], status);
}

fn heading(text: &str) -> Line<'static> {
    Line::from(Span::styled(
        text.to_string(),
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ))
}

fn build_git_lines(state: &GitViewState) -> Vec<Line<'static>> {
    let mut lines: Vec<Line<'static>> = Vec::new();
    let branch = state.branch.as_deref().unwrap_or("(detached HEAD)");
    lines.push(Line::from(format!("Branch: {branch}")));
    lines.push(Line::from(format!("Dirty files: {}", state.dirty_files)));
    lines.push(Line::from(""));

    lines.push(heading("── Recent Commits ──"));
    for line in &state.log_lines {
        lines.push(Line::from(line.clone()));
    }

    lines
}
