//! Step budget for a worker session's tool loop.
//!
//! The loop in [`super::run_session_steps`] used a hard-coded 50 steps. A
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
pub(super) const DEFAULT_WORKER_MAX_STEPS: usize = 200;

/// Environment override for the default step budget.
pub(super) const WORKER_MAX_STEPS_ENV: &str = "CODETETHER_WORKER_MAX_STEPS";

/// Resolves the step budget for a task from metadata, environment, or default.
///
/// # Examples
///
/// ```ignore
/// let budget = resolve_step_budget(&metadata);
/// assert!(budget >= 1);
/// ```
pub(super) fn resolve_step_budget(metadata: &serde_json::Map<String, serde_json::Value>) -> usize {
    resolve_with_env(metadata, std::env::var(WORKER_MAX_STEPS_ENV).ok().as_deref())
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
mod tests {
    use super::*;

    fn metadata(json: serde_json::Value) -> serde_json::Map<String, serde_json::Value> {
        json.as_object().cloned().unwrap_or_default()
    }

    #[test]
    fn defaults_when_nothing_configured() {
        assert_eq!(
            resolve_with_env(&metadata(serde_json::json!({})), None),
            DEFAULT_WORKER_MAX_STEPS
        );
    }

    #[test]
    fn task_metadata_wins_over_env() {
        let m = metadata(serde_json::json!({"max_steps": 12}));
        assert_eq!(resolve_with_env(&m, Some("400")), 12);
        let m = metadata(serde_json::json!({"step_budget": "30"}));
        assert_eq!(resolve_with_env(&m, Some("400")), 30);
    }

    #[test]
    fn env_used_when_metadata_absent_and_zero_is_rejected() {
        let m = metadata(serde_json::json!({}));
        assert_eq!(resolve_with_env(&m, Some("400")), 400);
        assert_eq!(resolve_with_env(&m, Some("0")), DEFAULT_WORKER_MAX_STEPS);
        assert_eq!(resolve_with_env(&m, Some("nope")), DEFAULT_WORKER_MAX_STEPS);
    }
}
