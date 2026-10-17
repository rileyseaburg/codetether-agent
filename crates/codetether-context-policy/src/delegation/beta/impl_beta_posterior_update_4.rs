//! update policy implementation.
use super::*;

impl BetaPosterior {
    /// Apply an outcome. `lambda ∈ [0, 1]` decays pseudo-counts.
    pub fn update(&mut self, outcome: bool, lambda: f64) {
        self.alpha *= lambda;
        self.beta *= lambda;
        if outcome {
            self.alpha += 1.0;
        } else {
            self.beta += 1.0;
        }
        self.n += 1;
        self.last_update = Utc::now();
    }
}
