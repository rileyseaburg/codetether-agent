/// Multi-dimensional coherence score for a branch.
#[derive(Debug, Clone, Copy, PartialEq)]
///
/// # Examples
/// ```
/// let score = codetether_swarm_core::collapse::CoherenceScore { compile_health: 1.0, test_health: 1.0, contract_alignment: 1.0, diff_conflict_risk: 0.0, velocity: 1.0, resource_health_score: 1.0, invariant_breaks: 0 };
/// assert!(score.collapse_score() > 0.9);
/// ```
pub struct CoherenceScore {
    pub compile_health: f32,
    pub test_health: f32,
    pub contract_alignment: f32,
    pub diff_conflict_risk: f32,
    pub velocity: f32,
    pub resource_health_score: f32,
    pub invariant_breaks: u32,
}

impl CoherenceScore {
    pub fn collapse_score(&self) -> f32 {
        // Penalize conflicts/invariant breaks and reward deterministic health signals.
        let weighted = (self.compile_health * 0.29)
            + (self.test_health * 0.14)
            + (self.contract_alignment * 0.18)
            + ((1.0 - self.diff_conflict_risk) * 0.14)
            + (self.velocity * 0.08)
            + (self.resource_health_score * 0.17);

        // Hard penalty once invariant breaks begin to accumulate.
        let invariant_penalty = (self.invariant_breaks as f32 * 0.1).clamp(0.0, 0.6);
        (weighted - invariant_penalty).clamp(0.0, 1.0)
    }
}
