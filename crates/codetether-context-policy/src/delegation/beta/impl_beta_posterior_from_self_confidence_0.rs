//! from self confidence policy implementation.
use super::*;

impl BetaPosterior {
    /// Seed a fresh posterior from self-declared confidence.
    pub fn from_self_confidence(c_self: f64, kappa: f64) -> Self {
        let c = c_self.clamp(0.0, 1.0);
        Self {
            alpha: kappa * c,
            beta: kappa * (1.0 - c),
            n: 0,
            c_self: c,
            kappa,
            last_update: Utc::now(),
        }
    }
}
