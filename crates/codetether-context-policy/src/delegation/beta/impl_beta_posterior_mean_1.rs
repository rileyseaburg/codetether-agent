//! mean policy implementation.
use super::*;

impl BetaPosterior {
    /// Posterior mean: `μ = α / (α + β)`.
    pub fn mean(&self) -> f64 {
        let t = self.alpha + self.beta;
        if t <= 0.0 { 0.0 } else { self.alpha / t }
    }
}
