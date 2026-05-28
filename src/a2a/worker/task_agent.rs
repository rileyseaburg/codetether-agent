//! Agent execution selector for claimed tasks.

use std::sync::Arc;

use anyhow::Result;

use crate::session::Session;

use super::{
    TaskContext, WorkerTaskRuntime, execute_session_with_policy, execute_swarm_with_policy,
};

pub(super) async fn execute_task_agent(
    session: &mut Session,
    runtime: &WorkerTaskRuntime,
    context: &TaskContext,
    agent_type: &str,
    output_callback: Arc<dyn Fn(String) + Send + Sync + 'static>,
) -> Result<(&'static str, Option<String>, Option<String>, Option<String>)> {
    if super::is_swarm_agent(agent_type) {
        return execute_swarm_with_policy(
            session,
            &context.prompt,
            context.model_tier.as_deref(),
            context.selected_model.clone(),
            &context.metadata,
            context.complexity_hint.as_deref(),
            context.worker_personality.as_deref(),
            context.target_agent_name.as_deref(),
            Some(&runtime.bus),
            Some(Arc::clone(&output_callback)),
        )
        .await
        .map(|(result, success)| {
            if success {
                (
                    "completed",
                    Some(result.text),
                    None,
                    Some(result.session_id),
                )
            } else {
                (
                    "failed",
                    Some(result.text),
                    Some("Swarm execution completed with failures".to_string()),
                    Some(result.session_id),
                )
            }
        });
    }
    execute_session_with_policy(
        session,
        &context.prompt,
        runtime.auto_approve,
        context.model_tier.as_deref(),
        Some(output_callback),
    )
    .await
    .map(|result| {
        (
            "completed",
            Some(result.text),
            None,
            Some(result.session_id),
        )
    })
}
