//! Approval popup geometry.

use ratatui::layout::Rect;

pub(super) fn popup(area: Rect) -> Rect {
    let width = area.width.saturating_sub(4).clamp(36, 120).min(area.width);
    let available = area.height.saturating_sub(4);
    let height = available.clamp(10, 32).min(area.height);
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    Rect::new(x, y, width, height)
}
