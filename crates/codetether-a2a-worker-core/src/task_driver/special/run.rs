//! Dispatch special worker task kinds after runtime policy checks.
use super::SpecialTaskHost;
use crate::task_config::TaskContext;
/// Handle clone/forage tasks and leave ordinary agent tasks to their executor.
///
/// # Arguments
/// * `host` - Runtime policy and special-task execution operations.
/// * `task` - Original task envelope.
/// * `title` - Task title supplied to forage execution.
/// * `context` - Parsed task metadata and selected model.
/// # Returns
/// None for ordinary tasks, or a completed/failed execution result for special tasks.
/// # Examples
/// ```rust,no_run
/// # async fn example(host: &impl codetether_a2a_worker_core::task_driver::special::SpecialTaskHost, task: &serde_json::Value, context: &codetether_a2a_worker_core::task_config::TaskContext) {
/// let result = codetether_a2a_worker_core::task_driver::special::dispatch_special_task(host, task, "title", context).await;
/// # }
/// ```

pub async fn dispatch_special_task<H: SpecialTaskHost>(
    host: &H,
    task: &serde_json::Value,
    title: &str,
    context: &TaskContext,
) -> Option<crate::outcome::TaskInner> {
    let policy_args = crate::task_config::policy_args::from_metadata(&context.metadata);
    if context.raw_agent.eq_ignore_ascii_case("clone_repo") {
        if let Some(blocked) = host.evaluate("bash", &policy_args).await {
            return Some(Ok(("failed", None, Some(blocked.output), None)));
        }
        return Some(
            host.clone_repo(task, &context.metadata)
                .await
                .map(|msg| ("completed", Some(msg), None, None))
                .or_else(|err| Ok(("failed", None, Some(format!("Error: {err}")), None))),
        );
    }
    if crate::is_forage_agent(&context.raw_agent) {
        if let Some(blocked) = host.evaluate("forage", &policy_args).await {
            return Some(Ok(("failed", None, Some(blocked.output), None)));
        }
        return Some(
            host.forage(
                title,
                &context.prompt,
                &context.metadata,
                context.selected_model.clone(),
            )
            .await
            .map(|msg| ("completed", Some(msg), None, None))
            .or_else(|err| Ok(("failed", None, Some(format!("Error: {err}")), None))),
        );
    }
    None
}
