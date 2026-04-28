//! Tests for [`BetaPosterior`] math.

#[cfg(test)]
mod tests {
    use super::super::beta::BetaPosterior;

    #[test]
    fn beta_update_increments_success_count() {
        let mut post = BetaPosterior::from_self_confidence(0.5, 2.0);
        post.update(true, 1.0);
        assert_eq!(post.n, 1);
        assert!((post.alpha - 2.0).abs() < 1e-9);
        assert!((post.beta - 1.0).abs() < 1e-9);
    }

    #[test]
    fn beta_score_penalises_uncertainty() {
        let mut thin = BetaPosterior::from_self_confidence(0.8, 2.0);
        let mut thick = BetaPosterior::from_self_confidence(0.5, 2.0);
        for _ in 0..100 { thick.update(true, 1.0); thick.update(false, 1.0); }
        thin.update(false, 1.0);
        assert!(thin.score(0.5) < thick.score(0.5));
    }
}
