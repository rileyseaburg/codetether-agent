//! beta score penalises uncertainty policy.
use super::*;

#[test]
pub(super) fn beta_score_penalises_uncertainty() {
    let mut thin = BetaPosterior::from_self_confidence(0.8, 2.0);
    let mut thick = BetaPosterior::from_self_confidence(0.5, 2.0);
    for _ in 0..100 {
        thick.update(true, 1.0);
        thick.update(false, 1.0);
    }
    thin.update(false, 1.0);
    assert!(thin.score(0.5) < thick.score(0.5));
}
