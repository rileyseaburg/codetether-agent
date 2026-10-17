//! delegate to policy implementation.
use super::*;

impl DelegationState {
    /// Pick a peer to delegate to, or None for self-execution.
    /// Applies margin: `score(peer) > score(local) + δ`.
    pub fn delegate_to<'a>(
        &self,
        local: &'a str,
        peers: &'a [&'a str],
        skill: &str,
        bucket: Bucket,
    ) -> Option<&'a str> {
        let local_score = self.score(local, skill, bucket).unwrap_or(0.0);
        let mut best: Option<(&str, f64)> = None;
        for peer in peers {
            if *peer == local {
                continue;
            }
            let s = self.score(peer, skill, bucket).unwrap_or(0.0);
            if s > local_score + self.config.delta {
                if best.map_or(true, |(_, b)| s > b) {
                    best = Some((peer, s));
                }
            }
        }
        best.map(|(p, _)| p)
    }
}
