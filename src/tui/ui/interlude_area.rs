//! Centered play-break geometry.

use ratatui::layout::Rect;

pub(super) fn popup(area: Rect) -> Rect {
    let width = area.width.min(31);
    let height = area.height.min(13);
    Rect::new(
        area.x + (area.width - width) / 2,
        area.y + (area.height - height) / 2,
        width,
        height,
    )
}
