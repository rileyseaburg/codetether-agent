//! Observation-driven scoring, pruning and promotion of speculative branches.
//!
//! Feed backend observations to [CollapseController::sample_observations].
//! [CollapsePolicy] supplies thresholds; [CollapseTick] reports evaluations,
//! kill decisions and the promoted task. This module executes no backend probes.
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
    time::Instant,
};
mod coherence_score;
pub use coherence_score::CoherenceScore;
mod branch_runtime_state;
pub use branch_runtime_state::BranchRuntimeState;
mod branch_evaluation;
pub use branch_evaluation::BranchEvaluation;
mod branch_observation;
pub use branch_observation::BranchObservation;
mod collapse_policy;
pub use collapse_policy::CollapsePolicy;
mod kill_decision;
pub use kill_decision::KillDecision;
mod collapse_tick;
pub use collapse_tick::CollapseTick;
mod conflict;
mod construct;
mod decisions;
mod evaluate;
mod health;
mod observe;
mod prune;
mod sample;
mod score;
#[cfg(test)]
mod tests;
mod thresholds;

/// First-class regulator that samples branch health and applies deterministic collapse policy.
#[derive(Debug)]
///
/// # Examples
/// ```
/// let mut controller = codetether_swarm_core::collapse::CollapseController::new(Default::default());
/// assert!(controller.sample_observations(&[]).evaluations.is_empty());
/// ```
pub struct CollapseController {
    policy: CollapsePolicy,
    first_seen_at: HashMap<String, Instant>,
    consecutive_compile_failures: HashMap<String, u32>,
    consecutive_resource_failures: HashMap<String, u32>,
    invariant_breaks: HashMap<String, u32>,
    promoted_subtask_id: Option<String>,
}
struct ObservationSnapshot {
    changed_files: HashMap<String, HashSet<String>>,
    changed_lines: HashMap<String, u32>,
    compile_ok: HashMap<String, bool>,
}

/// Branch planning for speculative execution strategies.
pub mod speculative;
