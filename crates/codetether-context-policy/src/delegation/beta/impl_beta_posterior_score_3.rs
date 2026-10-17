//! score policy implementation.
use super::*;

impl BetaPosterior {
    /// LCB risk-aware score: `μ − γ · √u`.
    pub fn score(&self, gamma: f64) -> f64 {
        self.mean() - gamma * self.variance().sqrt()
    }
}
