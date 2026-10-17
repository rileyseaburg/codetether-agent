//! reserves the actual output limit even above a quarter window derivation policy.
use super::*;

#[test]
pub(super) fn reserves_the_actual_output_limit_even_above_a_quarter_window() {
    assert_eq!(calculate(32_000, 16_000), 12_556);
    assert_eq!(calculate(32_000, 32_000), 0);
    assert_eq!(calculate(32_000, usize::MAX), 0);
}
