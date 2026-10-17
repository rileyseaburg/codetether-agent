//! default is incremental derivation policy.
use super::*;

#[test]
pub(super) fn default_is_incremental() {
    assert_eq!(DerivePolicy::default().kind(), "incremental");
}
