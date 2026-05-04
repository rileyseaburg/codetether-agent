//! Top-level dispatcher for [`render_chat_message`].

use ratatui::text::Line;

use super::formatted::render_formatted_message;
use super::image_file::{render_file, render_image};
use crate::tui::chat::message::{ChatMessage, MessageType};
use crate::tui::color_palette::ColorPalette;
use crate::tui::message_formatter::MessageFormatter;

pub fn render_chat_message(
    lines: &mut Vec<Line<'static>>,
    message: &ChatMessage,
    formatter: &MessageFormatter,
    palette: &ColorPalette,
) {
    let (label, icon) = match &message.message_type {
        MessageType::User => ("user", "▸ "),
        MessageType::Assistant => ("assistant", "◆ "),
        MessageType::System => ("system", "⚙ "),
        MessageType::Error => ("error", "✖ "),
        MessageType::Image { .. } => return render_image(lines, message),
        MessageType::File { path, size } => return render_file(lines, message, path, *size),
        MessageType::ToolCall { .. }
        | MessageType::ToolResult { .. }
        | MessageType::Thinking(_) => return,
    };
    let color = palette.get_message_color(label);
    render_formatted_message(lines, message, formatter, label, icon, color);
}
