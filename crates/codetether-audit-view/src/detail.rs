//! Selected audit entry panel.
use crate::AuditViewState;
use ratatui::{
    Frame,
    layout::Rect,
    style::Stylize,
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub(crate) fn render_detail(f: &mut Frame, state: &AuditViewState, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title(" Detail ");
    let Some(entry) = state.entries.get(state.selected) else {
        f.render_widget(
            Paragraph::new("No audit entries yet.".dim()).block(block),
            area,
        );
        return;
    };
    let body = crate::detail_lines::lines(entry);

    f.render_widget(
        Paragraph::new(body).block(block).wrap(Wrap { trim: false }),
        area,
    );
}
