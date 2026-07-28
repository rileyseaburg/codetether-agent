use super::*;

#[test]
fn z_index_orders_correctly() {
    let mut a = el(PositionType::Absolute);
    a.z_index = Some(2);
    let b = el(PositionType::Static);
    let mut c = el(PositionType::Relative);
    c.z_index = Some(-1);
    let order = ZIndexResolver::paint_order(&[a, b, c]);
    assert_eq!(
        order.iter().map(|r| r.index).collect::<Vec<_>>(),
        vec![2, 1, 0]
    );
}
