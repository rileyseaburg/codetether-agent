//! Audit panel layout and rendering orchestration.
use crate::AuditViewState;
use crate::{body::render_body, footer::render_footer, header::render_header};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

/// Render the audit view: list on the left, detail on the right.
///
/// # Arguments
/// * `f` - Target terminal frame.
/// * `state` - Cached entries, filter and selection.
/// * `area` - Available rectangle.
/// # Returns
/// Unit after rendering the header, list, detail and footer.
/// # Examples
/// ```no_run
/// # fn example(f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
/// let mut state = codetether_audit_view::AuditViewState::default();
/// codetether_audit_view::render_audit_view(f, &mut state, area);
/// # }
/// ```
pub fn render_audit_view(f: &mut Frame, state: &mut AuditViewState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .split(area);

    render_header(f, state, chunks[0]);
    render_body(f, state, chunks[1]);
    render_footer(f, state, chunks[2]);
}
