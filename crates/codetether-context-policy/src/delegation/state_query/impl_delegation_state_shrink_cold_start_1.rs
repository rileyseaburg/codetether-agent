//! shrink cold start policy implementation.
use super::*;

impl DelegationState {
    /// Shrink cold-start by pulling mass from neighbouring buckets
    /// (Hoeffding-based transfer learning, heavily simplified).
    pub fn shrink_cold_start(
        &mut self,
        agent: &str,
        skill: &str,
        target: Bucket,
        neighbours: &[Bucket],
        transfer_weight: f64,
    ) {
        let mut n_sum = 0u64;
        let mut a_sum = 0.0;
        let mut b_sum = 0.0;
        for &nb in neighbours {
            let key = Self::key(agent, skill, nb);
            if let Some(p) = self.beliefs.get(&key) {
                if p.n > 0 {
                    n_sum += p.n;
                    a_sum += p.alpha;
                    b_sum += p.beta;
                }
            }
        }
        if n_sum == 0 {
            return;
        }
        let post = self.ensure(agent, skill, target, 0.5);
        post.alpha += a_sum * transfer_weight;
        post.beta += b_sum * transfer_weight;
    }
}
