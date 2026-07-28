use super::*;

#[test]
fn relative_offsets_from_normal_position() {
    let mut e = el(PositionType::Relative);
    e.offsets.left = Some(5.0);
    e.offsets.top = Some(7.0);
    RelativePositioner::compute(&mut e);
    assert_eq!((e.computed_x, e.computed_y), (15.0, 27.0));
}

#[test]
fn absolute_uses_containing_block_offsets() {
    let mut e = el(PositionType::Absolute);
    e.offsets.left = Some(30.0);
    e.offsets.top = Some(40.0);
    let cb = ContainingBlock {
        rect: Rect {
            x: 100.0,
            y: 200.0,
            width: 300.0,
            height: 300.0,
        },
    };
    AbsolutePositioner::compute(&mut e, cb);
    assert_eq!((e.computed_x, e.computed_y), (130.0, 240.0));
}

#[test]
fn fixed_uses_viewport() {
    let mut e = el(PositionType::Fixed);
    e.offsets.right = Some(10.0);
    e.offsets.bottom = Some(20.0);
    FixedPositioner::compute(&mut e, ContainingBlock::viewport(800.0, 600.0));
    assert_eq!((e.computed_x, e.computed_y), (690.0, 530.0));
}
