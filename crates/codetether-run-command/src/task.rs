//! Task normalization and pasted-output validation.
use anyhow::Result;

/// Collapse whitespace in a task to single spaces.
/// # Arguments
/// * `task` - Original task text.
/// # Returns
/// The normalized task.
/// # Examples
/// ```
/// assert_eq!(codetether_run_command::normalize_go_task_input(" repair  api\n routes "), "repair api routes");
/// ```
pub fn normalize_go_task_input(task: &str) -> String {
    task.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn looks_like_pasted_go_run_output(task: &str) -> bool {
    let lower = task.to_ascii_lowercase();
    let markers = [
        "progress:",
        "iterations:",
        "feature branch:",
        "stories:",
        "incomplete stories:",
        "next steps:",
        "assessment is done and documented",
    ];

    let marker_hits = markers.iter().filter(|m| lower.contains(**m)).count();
    marker_hits >= 2 || (task.len() > 400 && lower.contains("next steps:"))
}

/// Normalize the task and reject blank or pasted-run-output input.
/// # Arguments
/// * `task` - User-supplied task text.
/// # Returns
/// The normalized task when accepted.
/// # Errors
/// Returns the existing validation message for empty tasks or detected run output.
/// # Examples
/// ```
/// assert_eq!(codetether_run_command::validate_easy_go_task(" repair api ").unwrap(), "repair api");
/// assert!(codetether_run_command::validate_easy_go_task("").is_err());
/// ```
pub fn validate_easy_go_task(task: &str) -> Result<String> {
    let normalized = normalize_go_task_input(task);
    if normalized.is_empty() {
        anyhow::bail!(
            "`/go` requires a task. Example: /go implement /v1/agent compatibility routes"
        );
    }

    if looks_like_pasted_go_run_output(&normalized) {
        anyhow::bail!(
            "`/go` received text that looks like prior run output/logs. \
Use a concise objective sentence instead, e.g. `/go implement /v1/agent/* compatibility routes`."
        );
    }

    Ok(normalized)
}
