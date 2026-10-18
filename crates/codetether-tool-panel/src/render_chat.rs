//! Top-level dispatcher for [`render_chat_message`].

use ratatui::text::Line;

use super::formatted::render_formatted_message;
use super::image_file::{render_file, render_image};
use codetether_chat_model::{ChatMessage, MessageType};
use codetether_message_formatter::MessageFormatter;
use codetether_theme::ColorPalette;

///
/// # Arguments
/// * `lines` - Destination for formatted output.
/// * `message` - Chat message to render.
/// * `formatter` - Markdown and content formatter.
/// * `palette` - Role colors.
///
/// # Returns
/// Appends message rows while leaving tool messages for the activity panel.
///
/// # Examples
/// ```
/// # fn example(message: &codetether_chat_model::ChatMessage) {
/// let mut lines = vec![];
/// let formatter = codetether_message_formatter::MessageFormatter::new(80);
/// codetether_tool_panel::render_chat_message(&mut lines, message, &formatter, &codetether_theme::ColorPalette::marketing());
/// # }
/// ```
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
        MessageType::Thinking(_) => ("system", "💭 "),
        MessageType::Image { .. } => return render_image(lines, message),
        MessageType::File { path, size } => return render_file(lines, message, path, *size),
        MessageType::ToolCall { .. } | MessageType::ToolResult { .. } => return,
    };
    let color = palette.get_message_color(label);
    render_formatted_message(lines, message, formatter, label, icon, color);
}
