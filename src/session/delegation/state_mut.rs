//! `DelegationState` mutation and scoring methods.

use super::beta::BetaPosterior;
use super::state::DelegationState;
use crate::session::relevance::Bucket;

impl DelegationState {
    /// Look up or create the posterior for `(agent, skill, bucket)`.
    pub fn ensure(&mut self, agent: &str, skill: &str, bucket: Bucket, c_self: f64) -> &mut BetaPosterior {
        let key = Self::key(agent, skill, bucket);
        let kappa = self.config.kappa;
        self.beliefs.entry(key).or_insert_with(|| BetaPosterior::from_self_confidence(c_self, kappa))
    }

    /// Current LCB score; None when no posterior exists.
    pub fn score(&self, agent: &str, skill: &str, bucket: Bucket) -> Option<f64> {
        let key = Self::key(agent, skill, bucket);
        self.beliefs.get(&key).map(|p| p.score(self.config.gamma))
    }

    /// Record an outcome. Seeds a neutral posterior if absent.
    pub fn update(&mut self, agent: &str, skill: &str, bucket: Bucket, outcome: bool) {
        let lambda = self.config.lambda;
        let post = self.ensure(agent, skill, bucket, 0.5);
        post.update(outcome, lambda);
    }

    /// Pick a peer to delegate to, or None for self-execution.
    /// Applies margin: `score(peer) > score(local) + δ`.
    pub fn delegate_to<'a>(
        &self, local: &'a str, peers: &'a [&'a str], skill: &str, bucket: Bucket,
    ) -> Option<&'a str> {
        let local_score = self.score(local, skill, bucket).unwrap_or(0.0);
        let mut best: Option<(&str, f64)> = None;
        for peer in peers {
            if *peer == local { continue; }
            let s = self.score(peer, skill, bucket).unwrap_or(0.0);
            if s > local_score + self.config.delta {
                if best.map_or(true, |(_, b)| s > b) { best = Some((peer, s)); }
            }
        }
        best.map(|(p, _)| p)
    }
}
