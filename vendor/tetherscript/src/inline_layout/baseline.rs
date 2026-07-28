//! Baseline and vertical-alignment calculations.

use super::inline_box::VerticalAlign;
use super::line_box::LineBox;
use super::measure::{ascent, descent};

/// Computed baseline metrics for one inline box.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AlignmentMetrics {
    pub ascent: f32,
    pub descent: f32,
    pub height: f32,
}

/// Align children in a line according to baseline and vertical-align.
pub fn align_line(line: &mut LineBox) {
    let mut above = 0.0_f32;
    let mut below = 0.0_f32;
    for child in &line.children {
        above = above.max(ascent(&child.inline_box));
        below = below.max(descent(&child.inline_box));
    }
    if line.children.is_empty() {
        return;
    }
    line.baseline = above;
    line.height = (above + below).max(line.height);

    for child in &mut line.children {
        let a = ascent(&child.inline_box);
        child.baseline = a;
        child.y = match child.inline_box.vertical_align {
            VerticalAlign::Baseline => line.baseline - a,
            VerticalAlign::Top => 0.0,
            VerticalAlign::Middle => (line.height - child.height) / 2.0,
            VerticalAlign::Bottom => line.height - child.height,
        };
    }
}
