//! update policy implementation.
use super::*;

impl DelegationState {
    /// Record an outcome. Seeds a neutral posterior if absent.
    ///
    /// `λ` is sourced from `CODETETHER_DELEGATION_LAMBDA` when set, falling
    /// back to [`DelegationConfig::lambda`]. Both paths are clamped to the
    /// documented `[0.9, 1.0)` range (Phase C step 32).
    ///
    /// [`DelegationConfig::lambda`]: crate::delegation::config::DelegationConfig::lambda
    pub fn update(&mut self, agent: &str, skill: &str, bucket: Bucket, outcome: bool) {
        let lambda = crate::delegation::lambda::env_lambda_override()
            .unwrap_or_else(|| crate::delegation::lambda::clamp_lambda(self.config.lambda));
        let post = self.ensure(agent, skill, bucket, 0.5);
        post.update(outcome, lambda);
    }
}
