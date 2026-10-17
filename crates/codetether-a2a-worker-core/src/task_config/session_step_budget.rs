//! Step budget for a worker session's tool loop.
//!
//! The loop in the host session loop used a hard-coded 50 steps. A
//! Forgejo "work on this issue" task (riley/spotlessbinco#5847) exhausted that
//! while still mapping the codebase, then reported `completed` with no commit,
//! so the Temporal workflow failed with nothing actionable on the issue.
//!
//! Resolution order: task metadata `max_steps` (or `step_budget`), then the
//! `CODETETHER_WORKER_MAX_STEPS` environment variable, then
//! [`DEFAULT_WORKER_MAX_STEPS`].

use super::metadata_usize;

/// Default step budget for a delegated worker session.
///
/// Repository-changing tasks routinely need well over 50 model turns
/// (inspection, edits, validation, commit, publish).
pub const DEFAULT_WORKER_MAX_STEPS: usize = 200;

/// Environment override for the default step budget.
pub const WORKER_MAX_STEPS_ENV: &str = "CODETETHER_WORKER_MAX_STEPS";

/// Resolves the step budget for a task from metadata, environment, or default.
///
/// # Examples
///
/// ```ignore
/// let budget = resolve_step_budget(&metadata);
/// assert!(budget >= 1);
/// ```
pub fn resolve_step_budget(metadata: &serde_json::Map<String, serde_json::Value>) -> usize {
    resolve_with_env(
        metadata,
        std::env::var(WORKER_MAX_STEPS_ENV).ok().as_deref(),
    )
}

fn resolve_with_env(
    metadata: &serde_json::Map<String, serde_json::Value>,
    env_value: Option<&str>,
) -> usize {
    metadata_usize(metadata, &["max_steps", "step_budget"])
        .or_else(|| env_value?.trim().parse::<usize>().ok())
        .filter(|steps| *steps > 0)
        .unwrap_or(DEFAULT_WORKER_MAX_STEPS)
}
#[cfg(test)]
#[path = "step_budget_tests.rs"]
mod tests;
