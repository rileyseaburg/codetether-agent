//! Structured repair prompt for malformed model-authored swarm plans.

/// Build a repair request for an invalid model-authored plan.
///
/// # Arguments
/// * `task` - Original task.
/// * `malformed` - Response that failed validation.
/// * `error` - Validation error to explain.
/// * `parallel` - Whether concurrent root tasks are required.
///
/// # Returns
/// Prompt text requesting corrected decomposition JSON.
///
/// # Examples
/// ```
/// let prompt = codetether_swarm_core::planning::repair_prompt::build("inspect", "{}", "empty", false);
/// assert!(prompt.contains("empty"));
/// ```
pub fn build(task: &str, malformed: &str, error: &str, parallel: bool) -> String {
    let concurrency = if parallel {
        " Include at least two dependency-free root subtasks that start concurrently."
    } else {
        ""
    };
    format!(
        "Repair this invalid swarm decomposition. Return JSON only, with a `subtasks` array. \
        Every dependency must exactly match an earlier subtask name and the graph must be acyclic.\
        {concurrency}\n\n\
Original task:\n{task}\n\nValidation error:\n{error}\n\nInvalid response:\n{malformed}"
    )
}
