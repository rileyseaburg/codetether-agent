//! Mutable task and agent state for swarm coordination.
//!
//! [CoordinatorState] owns task readiness, agent registration, completion history
//! and statistics. Provider discovery and request execution belong to the host.
use crate::{SubAgent, SubTask, SubTaskResult, SubTaskStatus, SwarmStats};
use std::collections::HashMap;
mod access;
mod lifecycle;
mod queries;
/// Task registry, agent registry, successful completion history and counters.
///
/// # Examples
/// ```
/// let state = codetether_swarm_core::coordinator::CoordinatorState::default();
/// assert!(state.is_complete());
/// assert!(state.all_subtasks().is_empty());
/// ```
pub struct CoordinatorState {
    subtasks: HashMap<String, SubTask>,
    subagents: HashMap<String, SubAgent>,
    completed: Vec<String>,
    stats: SwarmStats,
}
impl Default for CoordinatorState {
    fn default() -> Self {
        Self {
            subtasks: HashMap::new(),
            subagents: HashMap::new(),
            completed: Vec::new(),
            stats: SwarmStats::default(),
        }
    }
}
