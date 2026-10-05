//! Approval popup geometry.

use ratatui::layout::Rect;

pub(super) fn popup(area: Rect) -> Rect {
    let width = area.width.saturating_sub(2).clamp(36, 160).min(area.width);
    let height = area.height.saturating_sub(2).max(10).min(area.height);
    let x = area.x + area.width.saturating_sub(width) / 2;
    let y = area.y + area.height.saturating_sub(height) / 2;
    Rect::new(x, y, width, height)
}

#[cfg(test)]
#[path = "approval_overlay_layout_tests.rs"]
mod tests;
