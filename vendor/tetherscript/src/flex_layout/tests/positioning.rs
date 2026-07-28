use super::*;

#[test]
fn justify_center_offsets_items() {
    let mut c = FlexContainer::default();
    c.justify_content = JustifyContent::Center;
    let r = layout_flex(&c, &[item(20.0, 10.0)], cons(100.0, 20.0));
    assert_eq!(r[0].rect.x, 40.0);
}

#[test]
fn wrap_creates_multiple_lines() {
    let mut c = FlexContainer::default();
    c.wrap = FlexWrap::Wrap;
    c.gap.row_gap = 5.0;
    let r = layout_flex(
        &c,
        &[item(60.0, 10.0), item(60.0, 20.0)],
        cons(100.0, 100.0),
    );
    assert_eq!(r[0].rect.y, 0.0);
    assert_eq!(r[1].rect.y, 15.0);
}

#[test]
fn order_changes_visual_sequence_but_output_sorted_by_index() {
    let c = FlexContainer::default();
    let mut a = item(10.0, 10.0);
    let mut b = item(10.0, 10.0);
    a.order = 2;
    b.order = 1;
    let r = layout_flex(&c, &[a, b], cons(100.0, 20.0));
    assert_eq!(r[0].index, 0);
    assert_eq!(r[0].rect.x, 10.0);
    assert_eq!(r[1].index, 1);
    assert_eq!(r[1].rect.x, 0.0);
}
