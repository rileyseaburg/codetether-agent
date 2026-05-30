//! Execution of already-claimed tasks.

use anyhow::Result;

use crate::provenance::ClaimProvenance;

use super::{
    TaskContext, WorkerTaskRuntime, build_output_callback, build_task_context, execute_task_agent,
    finalize_task_result, prepare_task_session, sync_timeline_to_runtime, task_dispatch_special,
    task_timeline,
};

pub(super) async fn execute_claimed_task<'a>(
    runtime: &WorkerTaskRuntime,
    task: &'a serde_json::Value,
    task_id: &'a str,
    title: &'a str,
    claim_provenance: &ClaimProvenance,
    provider_keys: Option<serde_json::Value>,
    timeline: &mut task_timeline::TaskTimeline,
) -> Result<(&'static str, Option<String>, Option<String>, Option<String>)> {
    let context = build_task_context(task, title);
    timeline.checkpoint(task_timeline::TaskCheckpoint::MetadataParsed);
    if let Some(result) =
        task_dispatch_special::dispatch_special_task(runtime, task, title, &context).await
    {
        return result;
    }
    let (mut session, agent_type) = prepare_task_session(
        runtime,
        task_id,
        &context,
        timeline,
        claim_provenance,
        provider_keys,
    )
    .await?;
    timeline.checkpoint_with_detail(
        task_timeline::TaskCheckpoint::AgentStarting,
        Some(format!(
            "agent={} model={:?}",
            agent_type, session.metadata.model
        )),
    );
    sync_timeline_to_runtime(timeline, runtime).await;
    let output = build_output_callback(
        runtime.client.clone(),
        runtime.server.clone(),
        runtime.token.clone(),
        runtime.worker_id.clone(),
        task_id.to_string(),
        runtime.bus.clone(),
    );
    let (mut status, mut result, mut error, session_id) =
        execute_task_agent(&mut session, runtime, &context, &agent_type, output)
            .await
            .unwrap_or_else(|err| ("failed", None, Some(format!("Error: {err}")), None));
    timeline.checkpoint_with_detail(
        task_timeline::TaskCheckpoint::AgentDone,
        Some(format!("status={status}")),
    );
    sync_timeline_to_runtime(timeline, runtime).await;
    finalize_task_result(
        &mut session,
        task_id,
        &mut status,
        &mut result,
        &mut error,
        context.is_virtual_task,
        timeline,
    )
    .await?;
    Ok((
        status,
        result,
        error,
        Some(session_id.unwrap_or_else(|| session.id.clone())),
    ))
}
