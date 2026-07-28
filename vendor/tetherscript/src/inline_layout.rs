#![allow(dead_code)]
//! Inline layout engine for the tetherscript browser.
//!
//! Lays out inline-level content into CSS-like line boxes using a simple
//! monospace text metric. Self-contained so it can be wired into the
//! existing block layout engine later.

pub mod baseline;
pub mod break_rules;
pub mod engine;
pub mod inline_box;
pub mod line_box;
pub mod measure;

#[cfg(test)]
mod tests;

pub use baseline::{align_line, AlignmentMetrics};
pub use break_rules::{break_text_to_width, BreakToken};
pub use engine::{layout_inline, InlineLayoutEngine};
pub use inline_box::{InlineBox, InlineBoxKind, TextRun, VerticalAlign};
pub use line_box::{LineBox, PositionedInlineBox};
pub use measure::{measure_text, CHAR_WIDTH, DEFAULT_FONT_SIZE, LINE_HEIGHT_FACTOR};
