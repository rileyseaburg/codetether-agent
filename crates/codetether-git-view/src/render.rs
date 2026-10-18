//! Render the `/git` TUI view.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::GitViewState;
use crate::build_git_lines;

/// Render the git status view into `area` of `f`.
/// # Arguments
/// * `f` - Target frame.
/// * `area` - Available terminal rectangle.
/// * `state` - Previously captured Git state.
/// * `status` - Current status text.
/// * `render_input_preview` - Host renderer for the footer area.
/// # Returns
/// Unit after drawing the snapshot and footer.
/// # Examples
/// ```no_run
/// # fn example(f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
/// let state = codetether_git_view::GitViewState::default();
/// codetether_git_view::render_git_view(f, area, &state, "ready", |_, _, _| {});
/// # }
/// ```
pub fn render_git_view(
    f: &mut Frame,
    area: Rect,
    state: &GitViewState,
    status: &str,
    render_input_preview: impl FnOnce(&mut Frame, Rect, &str),
) {
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
