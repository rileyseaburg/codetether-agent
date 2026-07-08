//! List item renderer for bus log rows.
//!
//! The sender id is tinted with the agent's stable identity color
//! (see [`crate::tui::ui::agent_color`]) so multi-agent traffic is
//! visually attributable at a glance.

use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::ListItem,
};

use super::entry::BusLogEntry;
use crate::tui::ui::agent_color::agent_color;
use crate::tui::ui::gradient::rgb_supported;

pub(super) fn item(idx: usize, entry: &BusLogEntry, selected_index: usize) -> ListItem<'static> {
    let prefix = if idx == selected_index { "▶ " } else { "  " };
    let sender_color = agent_color(&entry.sender_id, rgb_supported());
    ListItem::new(Line::from(vec![
        Span::raw(prefix),
        Span::styled(
            format!("[{}] ", entry.kind),
            Style::default()
                .fg(entry.kind_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!("{} ", entry.timestamp)),
        Span::styled(
            entry.sender_id.clone(),
            Style::default()
                .fg(sender_color)
                .add_modifier(Modifier::DIM),
        ),
        Span::raw(format!(" {}", entry.summary)),
    ]))
}
