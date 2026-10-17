//! Branch-specification planning for speculative execution.
//!
//! [SpeculativeRunner] assigns ordered strategies to bounded branch counts.
//! [BranchSpec] describes planned work; backend execution remains host-owned.
use super::CollapsePolicy;
use anyhow::Result;
mod strategies;
pub use strategies::STRATEGY_PROMPTS;
mod branch;
pub use branch::BranchSpec;
mod construct;
mod plan;
/// Configuration for branch-plan construction and collapse policy.
///
/// # Examples
/// ```
/// let planner = codetether_swarm_core::collapse::speculative::SpeculativeRunner::new(3, vec![]);
/// assert_eq!(planner.branch_count, 3);
/// ```

pub struct SpeculativeRunner {
    pub branch_count: usize,
    pub strategies: Vec<String>,
    pub policy: CollapsePolicy,
}
