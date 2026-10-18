//! Streaming-capability markers and card list row styling.
use codetether_a2a_core::AgentCard;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::ListItem,
};

pub(crate) fn list_item(card: &AgentCard) -> ListItem<'static> {
    let dot = if card.capabilities.streaming {
        "● "
    } else {
        "○ "
    };
    let color = if card.capabilities.streaming {
        Color::Green
    } else {
        Color::Yellow
    };
    let line = Line::from(vec![
        Span::styled(dot, Style::default().fg(color)),
        Span::styled(
            card.name.clone(),
            Style::default().add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(card.url.clone(), Style::default().fg(Color::DarkGray)),
    ]);
    ListItem::new(line)
}
