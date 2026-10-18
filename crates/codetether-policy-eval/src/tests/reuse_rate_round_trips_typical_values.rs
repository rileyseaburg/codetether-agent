use crate::reuse_rate;

#[test]
fn reuse_rate_round_trips_typical_values() {
    assert!((reuse_rate(&[7, 10]) - 0.7).abs() < 1e-9);
    assert!((reuse_rate(&[10, 10]) - 1.0).abs() < 1e-9);
}
