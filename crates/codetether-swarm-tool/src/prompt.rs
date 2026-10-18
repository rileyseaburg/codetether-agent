//! Repository-aware system prompt for direct swarm workers.

use codetether_swarm_core::prompt_policy::{SystemPromptInput, system_prompt};
use std::path::Path;

/// Compose direct-worker instructions with repository and session policy.
/// # Arguments
/// * `task_id`, `specialty` — Worker identity and optional role.
/// * `working_dir` — Isolated or read-only parent workspace.
/// * `model`, `instruction` — Selected model and delegated task.
/// * `read_only`, `expects_changes` — Capability and delivery expectations.
/// * `append_session_instructions` — Native session instruction callback.
/// # Returns
/// The composed prompt ending with the direct-worker delegation restriction.
/// # Examples
/// ```no_run
/// let prompt = codetether_swarm_tool::prompt::build("review", None, std::path::Path::new("."), "model", "Review", true, false, |prompt| prompt);
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
    append_session_instructions: impl FnOnce(String) -> String,
) -> String {
    let directory = working_dir.to_string_lossy();
    let prompt = system_prompt(
        SystemPromptInput {
            specialty: specialty.unwrap_or("Generalist"),
            subtask_id: task_id,
            working_dir: &directory,
            model,
            instruction,
            context: "",
            line_limit: None,
            read_only,
            expects_changes,
        },
        append_session_instructions,
    );
    format!("{prompt}\n\nRecursive delegation is unavailable in this direct swarm worker.")
}
