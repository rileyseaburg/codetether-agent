//! Local Git observation adapter for shared speculative-collapse policy.
//!
//! [CollapseController::sample] gathers backend observations; scoring and decisions
//! are owned by the shared controller. Commands run only when a caller samples.
use anyhow::{Context, Result, anyhow};
pub use codetether_swarm_core::collapse::{
    BranchEvaluation, BranchObservation, BranchRuntimeState, CoherenceScore, CollapsePolicy,
    CollapseTick, KillDecision,
};
use std::{collections::HashSet, path::PathBuf, process::Command};
mod collapse_git_probe;
mod git;
mod sample;
/// Local Git observations combined with the shared branch-collapse policy.
/// # Examples
/// ```
/// let mut controller = codetether_swarm_git::CollapseController::new(Default::default());
/// assert!(controller.sample(&[]).unwrap().kills.is_empty());
/// ```
pub struct CollapseController {
    inner: codetether_swarm_core::collapse::CollapseController,
}
impl CollapseController {
    /// Construct the controller with shared policy thresholds.
    /// # Examples
    /// ```
    /// let _controller = codetether_swarm_git::CollapseController::new(Default::default());
    /// ```
    pub fn new(policy: CollapsePolicy) -> Self {
        Self {
            inner: codetether_swarm_core::collapse::CollapseController::new(policy),
        }
    }
    /// Evaluate backend-provided observations without collecting local probes.
    pub fn sample_observations(&mut self, observations: &[BranchObservation]) -> CollapseTick {
        self.inner.sample_observations(observations)
    }
}

impl std::fmt::Debug for CollapseController {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.inner, f)
    }
}
