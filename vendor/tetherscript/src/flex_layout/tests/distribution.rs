use super::*;

#[test]
fn row_grow_distributes_free_space() {
    let c = FlexContainer::default();
    let mut a = item(50.0, 10.0);
    let mut b = item(50.0, 10.0);
    a.flex_grow = 1.0;
    b.flex_grow = 3.0;
    let r = layout_flex(&c, &[a, b], cons(200.0, 20.0));
    assert_eq!(r[0].rect.width, 75.0);
    assert_eq!(r[1].rect.width, 125.0);
    assert_eq!(r[1].rect.x, 75.0);
}

#[test]
fn row_shrink_uses_flex_shrink() {
    let c = FlexContainer::default();
    let r = layout_flex(
        &c,
        &[item(100.0, 10.0), item(100.0, 10.0)],
        cons(100.0, 20.0),
    );
    assert_eq!(r[0].rect.width, 50.0);
    assert_eq!(r[1].rect.width, 50.0);
}

#[test]
fn flex_basis_zero_ignores_content_then_grows() {
    let c = FlexContainer::default();
    let mut a = item(100.0, 10.0);
    let mut b = item(100.0, 10.0);
    a.flex_basis = FlexBasis::Zero;
    b.flex_basis = FlexBasis::Zero;
    a.flex_grow = 1.0;
    b.flex_grow = 1.0;
    let r = layout_flex(&c, &[a, b], cons(120.0, 20.0));
    assert_eq!(r[0].rect.width, 60.0);
    assert_eq!(r[1].rect.width, 60.0);
}
