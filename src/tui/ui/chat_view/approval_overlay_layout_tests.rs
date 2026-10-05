//! Bounds and available-space coverage for approval popup geometry.

use ratatui::layout::Rect;

#[test]
fn approval_popup_uses_tall_terminal_without_32_row_cap() {
    let popup = super::popup(Rect::new(0, 0, 120, 60));
    assert_eq!(popup.height, 58);
    assert_eq!(popup.width, 118);
}

#[test]
fn approval_popup_stays_inside_tiny_and_offset_areas() {
    for width in [0, 1, 8, 36, 80, 200] {
        for height in [0, 1, 4, 10, 24, 60] {
            let area = Rect::new(5, 7, width, height);
            let popup = super::popup(area);
            assert!(popup.x >= area.x && popup.y >= area.y);
            assert!(popup.right() <= area.right());
            assert!(popup.bottom() <= area.bottom());
        }
    }
}
