//! Inspector border and vertical panel layout.
use crate::{render_metrics, render_tool_history};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders},
};

/// Render the inspector in the entire frame.
///
/// # Arguments
/// * `f` - Destination frame.
/// * `state` - Completion and tool values.
///
/// # Returns
/// Draws metrics, history, border, and footer.
///
/// # Examples
/// ```no_run
/// # fn draw(f: &mut ratatui::Frame) {
/// let view = codetether_inspector_view::InspectorView::default();
/// codetether_inspector_view::render_inspector(f, &view);
/// # }
/// ```
pub fn render_inspector(f: &mut Frame, state: &crate::InspectorView<'_>) {
    let area = f.area();
    let outer = Block::default()
        .borders(Borders::ALL)
        .title(" Inspector ")
        .title_style(Style::default().fg(Color::Cyan));
    let inner = outer.inner(area);
    f.render_widget(outer, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(8), Constraint::Min(6)])
        .split(inner);

    render_metrics(f, state, chunks[0]);
    render_tool_history(f, state, chunks[1]);

    crate::footer::render_footer(f, state, area);
}
