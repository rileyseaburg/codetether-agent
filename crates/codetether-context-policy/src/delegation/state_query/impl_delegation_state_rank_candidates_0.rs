//! rank candidates policy implementation.
use super::*;

impl DelegationState {
    /// Rank candidates by LCB; first on cold-start (no data).
    pub fn rank_candidates<'a>(
        &self,
        agents: &'a [&'a str],
        skill: &str,
        bucket: Bucket,
    ) -> Option<&'a str> {
        if agents.is_empty() {
            return None;
        }
        let mut best: Option<(&str, f64)> = None;
        let mut any_data = false;
        for &a in agents {
            if let Some(s) = self.score(a, skill, bucket) {
                any_data = true;
                if best.map_or(true, |(_, b)| s > b) {
                    best = Some((a, s));
                }
            }
        }
        if any_data {
            best.map(|(a, _)| a)
        } else {
            agents.first().copied()
        }
    }
}
