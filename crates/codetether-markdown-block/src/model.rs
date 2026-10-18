//! Result of classifying one line of Markdown.
use ratatui::text::Span;

/// Outcome of block-level detection for one line.
/// # Examples
/// ```
/// use codetether_markdown_block::{classify, Block};
/// match classify("plain") { Block::Plain => {}, Block::Styled(_) | Block::Prefixed { .. } => panic!("unexpected styling") }
/// ```
pub enum Block {
    /// Pass the entire line to inline formatting.
    Plain,
    /// Fully styled content requiring no inline pass.
    Styled(Vec<Span<'static>>),
    /// Styled prefix followed by text for inline formatting.
    Prefixed {
        /// Styled block marker.
        prefix: Vec<Span<'static>>,
        /// Remaining unformatted text.
        rest: String,
    },
}
