//! Render a chat message header + body with timestamp, icon, label, and usage.

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use super::super::status_bar::format_timestamp;
use super::format_meta::{format_latency, format_tokens};
use crate::tui::chat::message::ChatMessage;
use crate::tui::message_formatter::MessageFormatter;

pub(super) fn render_formatted_message(
    lines: &mut Vec<Line<'static>>,
    message: &ChatMessage,
    formatter: &MessageFormatter,
    label: &str,
    icon: &str,
    color: Color,
) {
    let timestamp = format_timestamp(message.timestamp);
    let dim = Style::default()
        .fg(Color::DarkGray)
        .add_modifier(ratatui::style::Modifier::DIM);
    let mut header_spans = vec![
        Span::styled(format!("[{timestamp}] "), dim),
        Span::styled(icon.to_string(), Style::default().fg(color).bold()),
        Span::styled(label.to_string(), Style::default().fg(color).bold()),
    ];
    if let Some(u) = message.usage.as_ref() {
        header_spans.push(Span::styled(
            format!(
                "  · {} in / {} out · {}",
                format_tokens(u.prompt_tokens),
                format_tokens(u.completion_tokens),
                format_latency(u.duration_ms),
            ),
            dim,
        ));
    }
    lines.push(Line::from(header_spans));
    let formatted = crate::tui::ui::chat_view::format_cache::format_message_cached(
        message,
        label,
        formatter,
        formatter.max_width(),
    );
    for line in formatted {
        let mut spans = vec![Span::styled("  ", Style::default().fg(color))];
        spans.extend(line.spans);
        lines.push(Line::from(spans));
    }
}
