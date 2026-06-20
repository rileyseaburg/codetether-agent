//! List item renderer for bus log rows.

use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::ListItem,
};

use super::entry::BusLogEntry;

pub(super) fn item(idx: usize, entry: &BusLogEntry, selected_index: usize) -> ListItem<'static> {
    let prefix = if idx == selected_index { "▶ " } else { "  " };
    ListItem::new(Line::from(vec![
        Span::raw(prefix),
        Span::styled(
            format!("[{}] ", entry.kind),
            Style::default()
                .fg(entry.kind_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!(
            "{} {} {}",
            entry.timestamp, entry.sender_id, entry.summary
        )),
    ]))
}
