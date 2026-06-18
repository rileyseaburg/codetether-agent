//! Block-level markdown styling for chat messages.
//!
//! Detects headers, lists, blockquotes, and rules; renders styled spans or a
//! styled prefix plus residual inline text for the caller's emphasis pass.

use ratatui::text::Span;

use super::MessageFormatter;

#[path = "message_formatter_block_detect.rs"]
mod detect;
pub(super) use detect::{Block, classify};

#[cfg(test)]
#[path = "message_formatter_block_tests.rs"]
mod tests;

/// Format one line: apply block-level markdown, then inline emphasis.
pub(super) fn format_line(fmt: &MessageFormatter, line: &str, role: &str) -> Vec<Span<'static>> {
    match classify(line) {
        Block::Styled(spans) => spans,
        Block::Prefixed { mut prefix, rest } => {
            prefix.extend(fmt.format_inline_text(&rest, role));
            prefix
        }
        Block::Plain => fmt.format_inline_text(line, role),
    }
}
