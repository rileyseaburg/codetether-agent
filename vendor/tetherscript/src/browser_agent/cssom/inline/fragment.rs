//! Inline fragment model for line-breaking layout.

/// A single inline-level fragment positioned within a line box.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InlineFragment {
    pub x: i64,
    pub y: i64,
    pub width: i64,
    pub height: i64,
    pub baseline_offset: i64,
    pub kind: FragmentKind,
}

/// The content kind of an inline fragment.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FragmentKind {
    Text { content: String, font_size: i64 },
    InlineBlock { tag: String },
    Image { src: String },
}

impl InlineFragment {
    /// Create a fragment with given dimensions and kind at origin (0,0).
    pub fn new(width: i64, height: i64, baseline_offset: i64, kind: FragmentKind) -> Self {
        Self {
            x: 0,
            y: 0,
            width,
            height,
            baseline_offset,
            kind,
        }
    }
}
