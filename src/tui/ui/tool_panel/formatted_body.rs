//! Bubble-vs-flat body dispatch for chat messages.
//!
//! On truecolor terminals, user messages render as right-aligned rounded
//! bubbles and assistant messages as gradient-gutter timelines. On plain
//! terminals the classic two-space indent is used.

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
};

use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::ui::chat_view::bubble::assistant_bubble_lines;
use crate::tui::ui::chat_view::bubble::user_bubble;
use crate::tui::ui::gradient::rgb_supported;

/// Append the message body using the bubble layout when supported.
pub(super) fn render_body(
    lines: &mut Vec<Line<'static>>,
    message: &ChatMessage,
    body: Vec<Line<'static>>,
    color: Color,
    panel_width: usize,
) {
    if !rgb_supported() {
        return render_flat(lines, body, color);
    }
    if matches!(message.message_type, MessageType::User) {
        let rows: Vec<String> = body
            .into_iter()
            .map(|l| l.spans.iter().map(|s| s.content.as_ref()).collect())
            .collect();
        lines.extend(user_bubble(&rows, panel_width));
    } else {
        lines.extend(assistant_bubble_lines(body));
    }
}

fn render_flat(lines: &mut Vec<Line<'static>>, body: Vec<Line<'static>>, color: Color) {
    for line in body {
        let mut spans = vec![Span::styled("  ", Style::default().fg(color))];
        spans.extend(line.spans);
        lines.push(Line::from(spans));
    }
}
