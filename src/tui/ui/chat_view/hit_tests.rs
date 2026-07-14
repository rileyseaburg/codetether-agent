use ratatui::{layout::Rect, text::Line};

use super::ChatHit;

#[test]
fn scrolled_click_maps_to_visible_row() {
    let mut hit = ChatHit::default();
    let visible = vec![Line::from("line 40"), Line::from("line 41")];
    hit.record_visible(Rect::new(2, 3, 20, 4), 40, &visible);

    assert_eq!(hit.line_at(3, 4), Some("line 40"));
    assert_eq!(hit.line_at(3, 5), Some("line 41"));
}
