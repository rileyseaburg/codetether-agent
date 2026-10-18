//! Agent identity and capability detail lines.
use crate::text::{field, yes_no};
use codetether_a2a_core::AgentCard;
use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

pub(crate) fn detail_lines(card: &AgentCard) -> Vec<Line<'static>> {
    vec![
        field("Name", card.name.clone()),
        Line::from(""),
        field("URL", card.url.clone()),
        Line::from(""),
        field("Version", card.version.clone()),
        field("Protocol", card.protocol_version.clone()),
        Line::from(""),
        Line::from(Span::styled(
            "─ Capabilities ─",
            Style::default().fg(Color::Cyan),
        )),
        field("Streaming", yes_no(card.capabilities.streaming)),
        field("Push Notif", yes_no(card.capabilities.push_notifications)),
        field(
            "Transitions",
            yes_no(card.capabilities.state_transition_history),
        ),
    ]
}
