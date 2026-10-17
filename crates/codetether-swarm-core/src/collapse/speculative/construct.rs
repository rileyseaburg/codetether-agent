use super::*;
impl SpeculativeRunner {
    pub fn new(branch_count: usize, strategies: Vec<String>) -> Self {
        Self {
            branch_count: branch_count.clamp(1, 8),
            strategies,
            policy: CollapsePolicy::default(),
        }
    }
}
