//! Native entry point for shared cached chat-line assembly.
mod host;
pub use super::drawn_lines::DrawnLines;
use crate::tui::{
    app::state::App, color_palette::ColorPalette, message_formatter::MessageFormatter,
};
/// Render focused or main-chat lines, returning cache ownership after drawing.
pub fn build_chat_lines(
    app: &mut App,
    max_width: usize,
    content_width: usize,
    formatter: &MessageFormatter,
    palette: &ColorPalette,
) -> DrawnLines {
    codetether_message_cache::pipeline::build(
        &mut host::Native {
            app,
            formatter,
            palette,
        },
        max_width,
        content_width,
    )
}
