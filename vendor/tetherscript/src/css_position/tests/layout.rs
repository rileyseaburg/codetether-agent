use super::*;

#[test]
fn layout_computes_mixed_positions() {
    let mut abs = el(PositionType::Absolute);
    abs.offsets.left = Some(1.0);
    abs.offsets.top = Some(2.0);
    let rel = {
        let mut r = el(PositionType::Relative);
        r.offsets.left = Some(3.0);
        r
    };
    let mut layout = PositionedLayout::new(vec![abs, rel], 500.0, 400.0);
    layout.compute();
    assert_eq!(
        (layout.elements[0].computed_x, layout.elements[0].computed_y),
        (1.0, 2.0)
    );
    assert_eq!(layout.elements[1].computed_x, 13.0);
}
