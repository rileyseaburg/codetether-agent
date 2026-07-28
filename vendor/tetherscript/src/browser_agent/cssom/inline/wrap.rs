//! Line breaking and inline layout algorithm.

use super::fragment::InlineFragment;
use super::line::LineBox;
mod split;
use split::split_text_fragments;

/// Break fragments into lines at word boundaries within the container width.
pub fn break_lines(fragments: &[InlineFragment], container_width: i64) -> Vec<LineBox> {
    let h = fragments
        .iter()
        .map(|f| f.height)
        .max()
        .unwrap_or(16)
        .max(1);
    layout_inline(&split_text_fragments(fragments), container_width, h)
}

/// Lay out pre-split fragments into lines of the given width and line height.
pub fn layout_inline(frags: &[InlineFragment], width: i64, line_height: i64) -> Vec<LineBox> {
    let mut lines = Vec::new();
    let mut line = LineBox::new(0, width);
    for frag in frags {
        if line.remaining_width() < frag.width && !line.is_empty() {
            lines.push(line.finalize());
            line = LineBox::new(lines.len() as i64 * line_height, width);
        }
        line.push(frag.clone());
    }
    if !line.is_empty() {
        lines.push(line.finalize());
    }
    lines
}
