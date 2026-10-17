//! variance policy implementation.
use super::*;

impl BetaPosterior {
    /// Posterior variance: `αβ / ((α+β)²(α+β+1))`.
    pub fn variance(&self) -> f64 {
        let t = self.alpha + self.beta;
        if t <= 0.0 {
            return 0.0;
        }
        (self.alpha * self.beta) / (t * t * (t + 1.0))
    }
}
