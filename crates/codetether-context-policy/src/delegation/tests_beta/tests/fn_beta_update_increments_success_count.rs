//! beta update increments success count policy.
use super::*;

#[test]
pub(super) fn beta_update_increments_success_count() {
    let mut post = BetaPosterior::from_self_confidence(0.5, 2.0);
    post.update(true, 1.0);
    assert_eq!(post.n, 1);
    assert!((post.alpha - 2.0).abs() < 1e-9);
    assert!((post.beta - 1.0).abs() < 1e-9);
}
