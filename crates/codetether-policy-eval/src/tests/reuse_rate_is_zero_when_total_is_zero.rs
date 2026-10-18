use crate::reuse_rate;

#[test]
fn reuse_rate_is_zero_when_total_is_zero() {
    assert_eq!(reuse_rate(&[0, 0]), 0.0);
}
