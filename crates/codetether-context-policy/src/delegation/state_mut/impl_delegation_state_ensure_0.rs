//! ensure policy implementation.
use super::*;

impl DelegationState {
    /// Look up or create the posterior for `(agent, skill, bucket)`.
    pub fn ensure(
        &mut self,
        agent: &str,
        skill: &str,
        bucket: Bucket,
        c_self: f64,
    ) -> &mut BetaPosterior {
        let key = Self::key(agent, skill, bucket);
        let kappa = self.config.kappa;
        self.beliefs
            .entry(key)
            .or_insert_with(|| BetaPosterior::from_self_confidence(c_self, kappa))
    }
}
