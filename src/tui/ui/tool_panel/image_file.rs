//! Image / file message renderers.

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use super::super::status_bar::format_timestamp;
use crate::tui::app::text::truncate_preview;
use crate::tui::chat::message::ChatMessage;

pub(super) fn render_image(lines: &mut Vec<Line<'static>>, message: &ChatMessage) {
    let timestamp = format_timestamp(message.timestamp);
    lines.push(Line::from(vec![
        Span::styled(format!("[{timestamp}] "), dim()),
        Span::styled("🖼️  image", Style::default().fg(Color::Cyan).italic()),
    ]));
    lines.push(Line::from(Span::styled(
        format!("  {}", truncate_preview(&message.content, 120)),
        Style::default().fg(Color::Cyan).dim(),
    )));
}

pub(super) fn render_file(
    lines: &mut Vec<Line<'static>>,
    message: &ChatMessage,
    path: &str,
    size: Option<u64>,
) {
    let timestamp = format_timestamp(message.timestamp);
    let size_label = size.map(|s| format!(" ({s} bytes)")).unwrap_or_default();
    lines.push(Line::from(vec![
        Span::styled(format!("[{timestamp}] "), dim()),
        Span::styled(
            format!("📎 file: {path}{size_label}"),
            Style::default().fg(Color::Yellow),
        ),
    ]));
}

fn dim() -> Style {
    Style::default()
        .fg(Color::DarkGray)
        .add_modifier(ratatui::style::Modifier::DIM)
}
