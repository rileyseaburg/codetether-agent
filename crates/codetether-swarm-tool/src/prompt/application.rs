//! Direct-worker prompt composition with mandatory goal-turn instructions.
//!
//! [`build`] adds the shared goal maintenance contract to repository-aware
//! direct-worker prompts while preserving their delegation restriction.
use std::path::Path;
/// Compose repository-aware worker instructions with the goal-turn contract.
/// # Arguments
/// `task_id` and `specialty` identify the worker; `working_dir` and `model`
/// select its execution context. `instruction` describes the task, and the
/// capability flags retain its access and change expectations.
/// # Returns
/// The worker prompt including goal maintenance and direct-worker restrictions.
/// # Examples
/// ```no_run
/// let prompt = codetether_swarm_tool::prompt::application::build(
///     "worker", None, std::path::Path::new("."), "model", "Review", true, false);
/// assert!(prompt.ends_with("Recursive delegation is unavailable in this direct swarm worker."));
/// ```
pub fn build(
    task_id: &str,
    specialty: Option<&str>,
    working_dir: &Path,
    model: &str,
    instruction: &str,
    read_only: bool,
    expects_changes: bool,
) -> String {
    super::build(
        task_id,
        specialty,
        working_dir,
        model,
        instruction,
        read_only,
        expects_changes,
        codetether_goal_runtime::turn_instructions::append,
    )
}
