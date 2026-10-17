//! score policy implementation.
use super::*;

impl DelegationState {
    /// Current LCB score; None when no posterior exists.
    pub fn score(&self, agent: &str, skill: &str, bucket: Bucket) -> Option<f64> {
        let key = Self::key(agent, skill, bucket);
        self.beliefs.get(&key).map(|p| p.score(self.config.gamma))
    }
}
