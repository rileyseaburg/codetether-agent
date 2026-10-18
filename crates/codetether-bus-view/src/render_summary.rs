//! Protocol summary panel renderer.

use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use super::{
    render_summary_data, render_summary_lines, state::BusLogState, summary::ProtocolSummary,
};

pub(super) fn render_summary_panel(
    f: &mut Frame,
    state: &BusLogState,
    area: Rect,
    summary: &ProtocolSummary,
) {
    let data = render_summary_data::data(summary, state);
    let panel = Paragraph::new(render_summary_lines::lines(&data))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Protocol Summary"),
        )
        .wrap(Wrap { trim: true });
    f.render_widget(panel, area);
}
