use ratatui::{prelude::*, widgets::*};

use crate::SessionPickerState;
use crate::bounds;

use super::rows;

/// Render recent sessions around the selected entry.
/// # Arguments
/// * `f` - Destination frame.
/// * `state` - Shared session listing and cursor state.
/// * `area` - Sidebar panel rectangle.
/// # Returns
/// Draws title/metadata rows, or the empty-state label.
/// # Examples
/// ```no_run
/// # fn draw(f: &mut ratatui::Frame) {
/// let area = f.area();
/// codetether_session_picker::recent::render(f, &Default::default(), area);
/// # }
/// ```
pub fn render(f: &mut Frame, state: &SessionPickerState, area: Rect) {
    let mut lines = Vec::new();
    let visible = area.height.saturating_sub(2) as usize / 2;
    let selected = state
        .selected_session
        .min(state.sessions.len().saturating_sub(1));
    let range = bounds(selected, state.sessions.len(), visible);
    for idx in range {
        let summary = &state.sessions[idx];
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
