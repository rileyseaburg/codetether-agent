//! calculation scales with context window derivation policy.
use super::*;

#[test]
pub(super) fn calculation_scales_with_context_window() {
    assert_eq!(calculate(32_000, 8_192), 19_584);
    assert_eq!(calculate(128_000, 8_192), 105_984);
    assert_eq!(calculate(256_000, 8_192), 221_184);
    assert_eq!(calculate(1_000_000, 8_192), 890_784);
}
