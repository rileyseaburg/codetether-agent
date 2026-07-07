//! Render a chat message: header line + bubble/flat body.

use ratatui::{style::Color, text::Line};

use super::formatted_body::render_body;
use super::formatted_header::build_header;
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
    lines.push(build_header(message, icon, label, color));
    let body = crate::tui::ui::chat_view::format_cache::format_message_cached(
        message,
        label,
        formatter,
        formatter.max_width(),
    );
    render_body(lines, message, body, color, formatter.max_width());
}
