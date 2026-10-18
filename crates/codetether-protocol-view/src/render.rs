//! Registry split-pane layout and detail selection.
use crate::{
    RegistryView,
    cards::collect_agent_cards,
    details::{detail_widget, empty_widget},
    list::list_widget,
};
use ratatui::{
    Frame,
    layout::Rect,
    widgets::{Block, Borders},
};

/// Draw registered cards and selected capability details.
/// # Arguments
/// * `f` - Target frame.
/// * `view` - Registered names, worker identity and selection state.
/// * `area` - Available terminal rectangle.
/// # Returns
/// Unit after drawing the list and details panes.
/// # Examples
/// ```no_run
/// # fn example(f: &mut ratatui::Frame, area: ratatui::layout::Rect, view: &codetether_protocol_view::RegistryView<'_>) {
/// codetether_protocol_view::render_protocol_registry(f, view, area);
/// # }
/// ```
pub fn render_protocol_registry(f: &mut Frame, view: &RegistryView<'_>, area: Rect) {
    let cards = collect_agent_cards(view);
    let chunks = ratatui::layout::Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints([
            ratatui::layout::Constraint::Percentage(40),
            ratatui::layout::Constraint::Percentage(60),
        ])
        .split(area);
    f.render_widget(list_widget(&cards), chunks[0]);
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Agent Details");
    let inner = block.inner(chunks[1]);
    f.render_widget(block, chunks[1]);
    let body = detail_widget(view, &cards).unwrap_or_else(empty_widget);
    f.render_widget(body, inner);
}
