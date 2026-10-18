//! Detail-pane renderer for the selected bus log entry.

use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Wrap},
};

use super::state::BusLogState;

pub(super) fn render_detail(f: &mut Frame, state: &BusLogState, area: Rect) {
    let detail = state
        .selected_entry()
        .map(|entry| entry.detail.clone())
        .unwrap_or_else(|| "No entry selected".to_string());
    let widget = Paragraph::new(detail)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Protocol Detail"),
        )
        .wrap(Wrap { trim: false })
        .scroll((state.detail_scroll.min(u16::MAX as usize) as u16, 0));
    f.render_widget(widget, area);
}
