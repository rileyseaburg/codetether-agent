//! Simple text and inline box measurement.

use super::inline_box::{InlineBox, InlineBoxKind};

/// Monospace character advance used by the initial inline layout engine.
pub const CHAR_WIDTH: f32 = 8.0;
/// Default font size used by [`TextRun`](super::inline_box::TextRun).
pub const DEFAULT_FONT_SIZE: f32 = 16.0;
/// CSS-normal-ish line-height multiplier.
pub const LINE_HEIGHT_FACTOR: f32 = 1.2;

/// Measure text width using an 8px-per-character monospace metric.
pub fn measure_text(text: &str) -> f32 {
    text.chars().count() as f32 * CHAR_WIDTH
}

/// Return `(width, height)` for an inline box.
pub fn measure_inline_box(b: &InlineBox) -> (f32, f32) {
    match &b.kind {
        InlineBoxKind::Text(run) => (measure_text(&run.text), run.font_size * LINE_HEIGHT_FACTOR),
        InlineBoxKind::InlineBlock { width, height } | InlineBoxKind::Image { width, height } => {
            (*width, *height)
        }
        InlineBoxKind::LineBreak => (0.0, 0.0),
    }
}

/// Ascent for text/atomic inline content.
pub fn ascent(b: &InlineBox) -> f32 {
    match &b.kind {
        InlineBoxKind::Text(run) => run.font_size * 0.8,
        InlineBoxKind::InlineBlock { height, .. } | InlineBoxKind::Image { height, .. } => *height,
        InlineBoxKind::LineBreak => 0.0,
    }
}

/// Descent for text/atomic inline content.
pub fn descent(b: &InlineBox) -> f32 {
    match &b.kind {
        InlineBoxKind::Text(run) => run.font_size * 0.2,
        InlineBoxKind::InlineBlock { .. }
        | InlineBoxKind::Image { .. }
        | InlineBoxKind::LineBreak => 0.0,
    }
}
