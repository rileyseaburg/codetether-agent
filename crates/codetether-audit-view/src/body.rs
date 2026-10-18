//! Audit view body rendering.
use crate::AuditViewState;
use crate::{detail::render_detail, list};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

pub(crate) fn render_body(f: &mut Frame, state: &mut AuditViewState, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(55), Constraint::Percentage(45)])
        .split(area);

    list::render(f, state, chunks[0]);
    render_detail(f, state, chunks[1]);
}
