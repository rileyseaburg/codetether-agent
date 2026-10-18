//! Block-level markdown styling for chat messages.
//!
//! Detects headers, lists, blockquotes, and rules; renders styled spans or a
//! styled prefix plus residual inline text for the caller's emphasis pass.

use ratatui::text::Span;

use super::MessageFormatter;

pub(crate) use codetether_markdown_block::{Block, classify};

/// Format one line: apply block-level markdown, then inline emphasis.
pub(crate) fn format_line(fmt: &MessageFormatter, line: &str, role: &str) -> Vec<Span<'static>> {
    match classify(line) {
        Block::Styled(spans) => spans,
        Block::Prefixed { mut prefix, rest } => {
            prefix.extend(fmt.format_inline_text(&rest, role));
            prefix
        }
        Block::Plain => fmt.format_inline_text(line, role),
    }
}
