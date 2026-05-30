//! Non-agent task dispatching (clone_repo, forage).

use anyhow::Result;

use super::{TaskContext, WorkerTaskRuntime, handle_clone_repo_task, handle_forage_task};

pub(super) async fn dispatch_special_task(
    runtime: &WorkerTaskRuntime,
    task: &serde_json::Value,
    title: &str,
    context: &TaskContext,
) -> Option<Result<(&'static str, Option<String>, Option<String>, Option<String>)>> {
    if context.raw_agent.eq_ignore_ascii_case("clone_repo") {
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
