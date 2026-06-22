//! Extracts colored byte-range spans from Rust source via tree-sitter.
//!
//! [`highlight_spans`] parses `source` with `tree-sitter-rust`, runs the
//! bundled `HIGHLIGHTS_QUERY`, and returns `(start_byte, end_byte, color)`
//! tuples for each captured node that maps to a color. Parsing failures yield
//! an empty span list so the caller renders unstyled text.

use streaming_iterator::StreamingIterator;

use super::capture_color::capture_color;

/// A colored byte range: `(start, end, rgb)`.
pub type Span = (usize, usize, (u8, u8, u8));

/// Returns highlight spans for Rust `source`, or empty on parse failure.
pub fn highlight_spans(source: &str) -> Vec<Span> {
    let mut parser = tree_sitter::Parser::new();
    if parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .is_err()
    {
        return Vec::new();
    }
    let Some(tree) = parser.parse(source, None) else {
        return Vec::new();
    };
    let lang = tree_sitter_rust::LANGUAGE.into();
    let Ok(query) = tree_sitter::Query::new(&lang, tree_sitter_rust::HIGHLIGHTS_QUERY) else {
        return Vec::new();
    };
    let names = query.capture_names();
    let mut cursor = tree_sitter::QueryCursor::new();
    let mut stream = cursor.matches(&query, tree.root_node(), source.as_bytes());
    let mut spans = Vec::new();
    while let Some(m) = stream.next() {
        for cap in m.captures {
            if let Some(color) = capture_color(names[cap.index as usize]) {
                let r = cap.node.byte_range();
                spans.push((r.start, r.end, color));
            }
        }
    }
    spans
}
