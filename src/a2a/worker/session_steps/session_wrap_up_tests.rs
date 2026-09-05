//! Worker-budget wrap-up boundary regressions.

use super::wrap_up_step;

#[test]
fn wrap_up_reserves_fraction_with_floor() {
    assert_eq!(wrap_up_step(200), 170);
    assert_eq!(wrap_up_step(100), 85);
    assert_eq!(wrap_up_step(20), 12);
    assert_eq!(wrap_up_step(10), 2);
    assert_eq!(wrap_up_step(2), 1);
    assert_eq!(wrap_up_step(1), 1);
}
