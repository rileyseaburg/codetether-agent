/// Deterministic policy knobs for collapse behavior.
#[derive(Debug, Clone, Copy)]
///
/// # Examples
/// ```
/// let policy = codetether_swarm_core::collapse::CollapsePolicy::default();
/// assert_eq!(policy.max_invariant_breaks, 2);
/// ```
pub struct CollapsePolicy {
    pub max_invariant_breaks: u32,
    pub max_consecutive_compile_failures: u32,
    pub max_consecutive_resource_failures: u32,
    pub min_collapse_score: f32,
    pub min_score_gap_for_prune: f32,
    pub min_resource_health_score: f32,
}

impl Default for CollapsePolicy {
    fn default() -> Self {
        Self {
            max_invariant_breaks: 2,
            max_consecutive_compile_failures: 1,
            max_consecutive_resource_failures: 2,
            min_collapse_score: 0.30,
            min_score_gap_for_prune: 0.15,
            min_resource_health_score: 0.35,
        }
    }
}
