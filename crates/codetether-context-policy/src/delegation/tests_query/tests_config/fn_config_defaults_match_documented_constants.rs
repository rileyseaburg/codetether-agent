//! config defaults match documented constants policy.
use super::*;

#[test]
pub(super) fn config_defaults_match_documented_constants() {
    let cfg = DelegationConfig::default();
    assert!((cfg.gamma - DEFAULT_GAMMA).abs() < 1e-9);
    assert!((cfg.delta - DEFAULT_DELTA).abs() < 1e-9);
    assert!((cfg.kappa - DEFAULT_KAPPA).abs() < 1e-9);
    assert!((cfg.lambda - DEFAULT_LAMBDA).abs() < 1e-9);
    assert!(!cfg.enabled);
}
