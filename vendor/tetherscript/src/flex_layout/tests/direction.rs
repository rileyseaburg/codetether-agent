use super::*;

#[test]
fn column_direction_uses_vertical_main_axis() {
    let mut c = FlexContainer::default();
    c.direction = FlexDirection::Column;
    let r = layout_flex(
        &c,
        &[item(10.0, 20.0), item(10.0, 30.0)],
        cons(100.0, 100.0),
    );
    assert_eq!(r[0].rect.y, 0.0);
    assert_eq!(r[1].rect.y, 20.0);
    assert_eq!(r[1].rect.height, 30.0);
}

#[test]
fn row_reverse_places_first_item_at_end() {
    let mut c = FlexContainer::default();
    c.direction = FlexDirection::RowReverse;
    let r = layout_flex(&c, &[item(20.0, 10.0), item(20.0, 10.0)], cons(100.0, 20.0));
    assert_eq!(r[0].rect.x, 80.0);
    assert_eq!(r[1].rect.x, 60.0);
}
