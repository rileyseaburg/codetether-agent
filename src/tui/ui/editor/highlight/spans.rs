//! Extracts colored byte-range spans from Rust source via tree-sitter.
//!
//! [`highlight_spans`] maps the RLM tree-sitter oracle's neutral byte-range
//! captures to editor colors. Parsing failures yield an empty span list so the
//! caller renders unstyled text.

use super::capture_color::capture_color;
use crate::rlm::TreeSitterOracle;

/// A colored byte range: `(start, end, rgb)`.
pub type Span = (usize, usize, (u8, u8, u8));

/// Returns highlight spans for Rust `source`, or empty on parse failure.
pub fn highlight_spans(source: &str) -> Vec<Span> {
    let mut oracle = TreeSitterOracle::new(source.to_string());
    oracle
        .rust_highlight_captures()
        .unwrap_or_default()
        .into_iter()
        .filter_map(|capture| {
            capture_color(&capture.name).map(|color| (capture.start_byte, capture.end_byte, color))
        })
        .collect()
}
