//! Rust syntax highlighting for the editor backend.
//!
//! [`HighlightMap`] precomputes a per-byte color table from tree-sitter
//! highlight spans so the renderer can color each character in O(1). Build it
//! once per document edit; look up colors while rendering cells.

mod capture_color;
mod spans;

pub use capture_color::capture_color;
pub use spans::{highlight_spans, Span};

/// Per-byte color lookup for a Rust document.
#[derive(Debug, Clone, Default)]
pub struct HighlightMap {
    colors: Vec<Option<(u8, u8, u8)>>,
}

impl HighlightMap {
    /// Builds a highlight map for Rust `source`.
    pub fn rust(source: &str) -> Self {
        let mut colors = vec![None; source.len()];
        for (start, end, color) in highlight_spans(source) {
            for slot in colors.iter_mut().take(end).skip(start) {
                *slot = Some(color);
            }
        }
        Self { colors }
    }

    /// Color for the character at byte offset `byte`, if any.
    pub fn color_at(&self, byte: usize) -> Option<(u8, u8, u8)> {
        self.colors.get(byte).copied().flatten()
    }
}
