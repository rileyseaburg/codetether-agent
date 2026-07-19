//! Task context builder helpers.

#[path = "task_author_binding.rs"]
mod task_author_binding;

use super::task_context_helpers;

pub(super) use super::task_context_struct::TaskContext;

pub(super) fn build_task_context(task: &serde_json::Value, title: &str) -> TaskContext {
    let metadata = super::task_metadata(task);
    let complexity_hint = super::metadata_str(&metadata, &["complexity"]);
    let model_tier =
        task_context_helpers::tier_from_metadata(&metadata, complexity_hint.as_deref());
    let workspace_id = task_context_helpers::context_workspace_id(task, &metadata);
    let resume_session_id = task_author_binding::resume_session_id(&metadata);
    let context_id = super::metadata_str(&metadata, &["context_id", "conversation_id"]);
    let preserve_session_workspace =
        task_author_binding::preserve_workspace(&metadata, &resume_session_id);
    TaskContext {
        worker_personality: super::metadata_str(
            &metadata,
            &["worker_personality", "personality", "agent_personality"],
        ),
        target_agent_name: super::metadata_str(&metadata, &["target_agent_name", "agent_name"]),
        selected_model: task_context_helpers::context_selected_model(task, &metadata),
        raw_agent: super::task_str(task, "agent_type")
            .or_else(|| super::task_str(task, "agent"))
            .unwrap_or("build")
            .to_string(),
        prompt: super::task_str(task, "prompt")
            .or_else(|| super::task_str(task, "description"))
            .unwrap_or(title)
            .to_string(),
        is_virtual_task: workspace_id
            .as_deref()
            .is_some_and(|v| v == "global" || v.is_empty()),
        metadata,
        resume_session_id,
        context_id,
        preserve_session_workspace,
        complexity_hint,
        model_tier,
        workspace_id,
    }
}

#[cfg(test)]
#[path = "task_context_tests.rs"]
mod tests;
