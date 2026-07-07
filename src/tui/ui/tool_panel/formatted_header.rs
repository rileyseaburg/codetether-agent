//! Header line (timestamp, icon, label, usage) for a chat message.

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use super::super::status_bar::format_timestamp;
use super::format_meta::{format_latency, format_tokens};
use crate::tui::chat::message::ChatMessage;

/// Build the `[ts] icon label · usage` header line.
pub(super) fn build_header(
    message: &ChatMessage,
    icon: &str,
    label: &str,
    color: Color,
) -> Line<'static> {
    let dim = Style::default()
        .fg(Color::DarkGray)
        .add_modifier(ratatui::style::Modifier::DIM);
    let ts = format_timestamp(message.timestamp);
    let mut spans = vec![
        Span::styled(format!("[{ts}] "), dim),
        Span::styled(icon.to_string(), Style::default().fg(color).bold()),
        Span::styled(label.to_string(), Style::default().fg(color).bold()),
    ];
    if let Some(u) = message.usage.as_ref() {
        spans.push(Span::styled(
            format!(
                "  · {} in / {} out · {}",
                format_tokens(u.prompt_tokens),
                format_tokens(u.completion_tokens),
                format_latency(u.duration_ms),
            ),
            dim,
        ));
    }
    Line::from(spans)
}
