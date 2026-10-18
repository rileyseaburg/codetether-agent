//! Fuzzy finder area presentation.
use ratatui::layout::Rect;

pub(crate) fn popup_area(area: Rect) -> Rect {
    let width = area.width.min(90).max(area.width.min(50));
    let height = area.height.min(20).max(area.height.min(8));
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width, height)
}
