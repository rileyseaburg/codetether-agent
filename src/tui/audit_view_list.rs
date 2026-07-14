//! Visible-window renderer for Audit events.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use super::AuditViewState;

#[path = "audit_view_list_row.rs"]
mod row;

pub(super) fn render(f: &mut Frame, state: &AuditViewState, area: Rect) {
    let count = area.height.saturating_sub(2) as usize;
    let (start, end) = visible_range(state.entries.len(), state.selected, count);
    let items: Vec<ListItem> = state.entries[start..end]
        .iter()
        .map(|entry| ListItem::new(row::format(entry)))
        .collect();
    let mut selection = ListState::default();
    if !items.is_empty() {
        selection.select(Some(state.selected - start));
    }
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Events "))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");
    f.render_stateful_widget(list, area, &mut selection);
}

fn visible_range(len: usize, selected: usize, count: usize) -> (usize, usize) {
    let selected = selected.min(len.saturating_sub(1));
    let start = selected.saturating_sub(count.saturating_sub(1));
    (start, start.saturating_add(count).min(len))
}

#[cfg(test)]
#[path = "audit_view_list_tests.rs"]
mod tests;
