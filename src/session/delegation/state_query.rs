//! `DelegationState` query and ranking helpers.

use super::state::DelegationState;
use crate::session::relevance::Bucket;

impl DelegationState {
    /// Rank candidates by LCB; first on cold-start (no data).
    pub fn rank_candidates<'a>(
        &self, agents: &'a [&'a str], skill: &str, bucket: Bucket,
    ) -> Option<&'a str> {
        if agents.is_empty() { return None; }
        let mut best: Option<(&str, f64)> = None;
        let mut any_data = false;
        for &a in agents {
            if let Some(s) = self.score(a, skill, bucket) {
                any_data = true;
                if best.map_or(true, |(_, b)| s > b) { best = Some((a, s)); }
            }
        }
        if any_data { best.map(|(a, _)| a) } else { agents.first().copied() }
    }

    /// Shrink cold-start by pulling mass from neighbouring buckets
    /// (Hoeffding-based transfer learning, heavily simplified).
    pub fn shrink_cold_start(
        &mut self, agent: &str, skill: &str, target: Bucket,
        neighbours: &[Bucket], transfer_weight: f64,
    ) {
        let mut n_sum = 0u64;
        let mut a_sum = 0.0;
        let mut b_sum = 0.0;
        for &nb in neighbours {
            let key = Self::key(agent, skill, nb);
            if let Some(p) = self.beliefs.get(&key) {
                if p.n > 0 { n_sum += p.n; a_sum += p.alpha; b_sum += p.beta; }
            }
        }
        if n_sum == 0 { return; }
        let post = self.ensure(agent, skill, target, 0.5);
        post.alpha += a_sum * transfer_weight;
        post.beta += b_sum * transfer_weight;
    }
}
