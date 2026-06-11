//! Non-agent task dispatching (clone_repo, forage).

use anyhow::Result;

use super::{TaskContext, WorkerTaskRuntime, handle_clone_repo_task, handle_forage_task};

pub(super) async fn dispatch_special_task(
    runtime: &WorkerTaskRuntime,
    task: &serde_json::Value,
    title: &str,
    context: &TaskContext,
) -> Option<Result<(&'static str, Option<String>, Option<String>, Option<String>)>> {
    let policy_args = super::task_policy_args::from_metadata(&context.metadata);
    if context.raw_agent.eq_ignore_ascii_case("clone_repo") {
        if let Some(blocked) =
            crate::runtime_policy::evaluate_tool_invocation("bash", &policy_args).await
        {
            return Some(Ok(("failed", None, Some(blocked.output), None)));
        }
        return Some(
            handle_clone_repo_task(
                &runtime.client,
                &runtime.server,
                &runtime.token,
                &runtime.worker_id,
                task,
                &context.metadata,
            )
            .await
            .map(|msg| ("completed", Some(msg), None, None))
            .or_else(|err| Ok(("failed", None, Some(format!("Error: {err}")), None))),
        );
    }
    if super::is_forage_agent(&context.raw_agent) {
        if let Some(blocked) =
            crate::runtime_policy::evaluate_tool_invocation("forage", &policy_args).await
        {
            return Some(Ok(("failed", None, Some(blocked.output), None)));
        }
        return Some(
            handle_forage_task(
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
