//! Per-(agent, skill, bucket) Beta-Bernoulli posterior.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Beta-Bernoulli posterior for a single (agent, skill, bucket) cell.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::delegation::BetaPosterior;
///
/// let mut p = BetaPosterior::from_self_confidence(0.7, 2.0);
/// p.update(true, 1.0);
/// assert_eq!(p.n, 1);
/// assert!(p.score(0.5).is_finite());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BetaPosterior {
    /// Success pseudo-count + weak prior.
    pub alpha: f64,
    /// Failure pseudo-count + weak prior.
    pub beta: f64,
    /// Real observations so far.
    pub n: u64,
    /// Self-declared prior confidence `[0, 1]`.
    pub c_self: f64,
    /// Weak-prior strength multiplier.
    pub kappa: f64,
    /// Timestamp of the last update.
    pub last_update: DateTime<Utc>,
}

impl BetaPosterior {
    /// Seed a fresh posterior from self-declared confidence.
    pub fn from_self_confidence(c_self: f64, kappa: f64) -> Self {
        let c = c_self.clamp(0.0, 1.0);
        Self { alpha: kappa * c, beta: kappa * (1.0 - c), n: 0, c_self: c, kappa, last_update: Utc::now() }
    }

    /// Posterior mean: `μ = α / (α + β)`.
    pub fn mean(&self) -> f64 {
        let t = self.alpha + self.beta;
        if t <= 0.0 { 0.0 } else { self.alpha / t }
    }

    /// Posterior variance: `αβ / ((α+β)²(α+β+1))`.
    pub fn variance(&self) -> f64 {
        let t = self.alpha + self.beta;
        if t <= 0.0 { return 0.0; }
        (self.alpha * self.beta) / (t * t * (t + 1.0))
    }

    /// LCB risk-aware score: `μ − γ · √u`.
    pub fn score(&self, gamma: f64) -> f64 { self.mean() - gamma * self.variance().sqrt() }

    /// Apply an outcome. `lambda ∈ [0, 1]` decays pseudo-counts.
    pub fn update(&mut self, outcome: bool, lambda: f64) {
        self.alpha *= lambda;
        self.beta *= lambda;
        if outcome { self.alpha += 1.0; } else { self.beta += 1.0; }
        self.n += 1;
        self.last_update = Utc::now();
    }
}
