use super::TaskCheckpoint;

pub(super) fn log_checkpoint(
    task_id: &str,
    timeout_secs: u64,
    cp: TaskCheckpoint,
    elapsed_ms: u64,
    budget: f64,
    remaining_secs: f64,
    detail: Option<&str>,
) {
    tracing::info!(task_id = %task_id, checkpoint = cp.as_str(), elapsed_ms, timeout_secs, budget_pct = format!("{:.1}%", budget), detail = detail.unwrap_or(""), "[timeline] checkpoint reached");
    if budget >= 90.0 {
        tracing::warn!(task_id = %task_id, checkpoint = cp.as_str(), budget_pct = format!("{:.1}%", budget), remaining_secs = format!("{:.1}", remaining_secs), "DEADLINE APPROACHING - {:.0}% of time budget consumed", budget);
    } else if budget >= 75.0 {
        tracing::warn!(task_id = %task_id, budget_pct = format!("{:.1}%", budget), "Task consumed {:.0}% of time budget", budget);
    }
}
