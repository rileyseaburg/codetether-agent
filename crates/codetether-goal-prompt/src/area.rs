//! Modal size limits and centering.
use ratatui::layout::Rect;

pub(super) fn popup_area(area: Rect) -> Rect {
    let width = area.width.min(72).max(area.width.min(40));
    let height = area.height.min(16).max(area.height.min(10));
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect::new(x, y, width, height)
}
