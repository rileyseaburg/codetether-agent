//! Select interactive handoff or lazy headless execution.
use super::*;

/// Run `prompt` for task `task_id` and return the assistant's final text.
///
/// # Arguments
///
/// Supply the task table, task id, optional context id, sender, prompt and a
/// lazy headless future. The future is only polled when no inbox is attached.
///
/// # Returns
///
/// The interactive reply or the supplied headless result.
///
/// # Errors
///
/// Returns an interactive handoff error or the error from headless execution.
///
/// # Examples
///
/// ```rust,no_run
/// # async fn example(tasks: &dashmap::DashMap<String, codetether_a2a_core::Task>) -> anyhow::Result<()> {
/// let answer = codetether_a2a_http::inbound_turn::execute_turn(
///     tasks, "task-1", None, "peer", "hello", async { Ok("hi".to_string()) },
/// ).await?;
/// assert!(!answer.is_empty());
/// # Ok(()) }
/// ```
pub async fn execute_turn(
    tasks: &DashMap<String, Task>,
    task_id: &str,
    context_id: Option<&str>,
    from: &str,
    prompt: &str,
    headless: impl Future<Output = Result<String>>,
) -> Result<String> {
    if live_inbox::is_attached() {
        return live(tasks, task_id, context_id, from, prompt).await;
    }
    headless.await
}
