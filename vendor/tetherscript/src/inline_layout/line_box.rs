//! Line box and positioned inline content types.

use super::inline_box::InlineBox;

/// An inline box with coordinates relative to its line box.
#[derive(Clone, Debug, PartialEq)]
pub struct PositionedInlineBox {
    pub inline_box: InlineBox,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub baseline: f32,
}

/// A single line containing positioned inline-level boxes.
#[derive(Clone, Debug, PartialEq)]
pub struct LineBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub baseline: f32,
    pub children: Vec<PositionedInlineBox>,
}

impl LineBox {
    pub fn new(y: f32) -> Self {
        Self {
            x: 0.0,
            y,
            width: 0.0,
            height: 0.0,
            baseline: 0.0,
            children: Vec::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }
}
