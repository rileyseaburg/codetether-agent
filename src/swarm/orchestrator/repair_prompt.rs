//! Structured repair prompt for malformed model-authored swarm plans.

pub(super) fn build(task: &str, malformed: &str, error: &str) -> String {
    format!(
        "Repair this invalid swarm decomposition. Return JSON only, with a `subtasks` array. \
Every dependency must exactly match an earlier subtask name and the graph must be acyclic.\n\n\
Original task:\n{task}\n\nValidation error:\n{error}\n\nInvalid response:\n{malformed}"
    )
}
