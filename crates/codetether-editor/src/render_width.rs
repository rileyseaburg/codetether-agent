//! Gutter and available text width calculation.
use super::backend::EditorBackend;

/// Number of digits needed to show the largest line number.
pub(super) fn gutter_width(line_count: usize) -> usize {
    line_count.max(1).to_string().len()
}

/// Usable text columns inside `area_width` after borders and the gutter.
pub fn text_width<B: EditorBackend>(backend: &B, area_width: u16) -> usize {
    let gutter = gutter_width(backend.line_count()) + 1;
    (area_width as usize).saturating_sub(2 + gutter)
}
