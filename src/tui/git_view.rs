//! Render the `/git` TUI view.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::tui::app::state::git_state::GitViewState;
use crate::tui::git_lines::build_git_lines;
use crate::tui::input::render_input_preview;

/// Render the git status view into `area` of `f`.
pub fn render_git_view(f: &mut Frame, area: Rect, state: &GitViewState, status: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Length(3)])
        .split(area);

    let widget = Paragraph::new(build_git_lines(state))
        .block(Block::default().borders(Borders::ALL).title("Git"))
        .wrap(Wrap { trim: false });
    f.render_widget(widget, chunks[0]);
    render_input_preview(f, chunks[1], status);
}
