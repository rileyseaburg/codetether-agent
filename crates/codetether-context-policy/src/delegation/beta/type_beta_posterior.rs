//! Shared BetaPosterior contract.
use super::*;

/// Beta-Bernoulli posterior for a single (agent, skill, bucket) cell.
///
/// # Examples
///
/// ```rust
/// use codetether_context_policy::delegation::BetaPosterior;
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
