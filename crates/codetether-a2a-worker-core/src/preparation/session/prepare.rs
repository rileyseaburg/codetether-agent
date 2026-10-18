use super::{Host, Loader, SessionState};
use crate::{task_config::TaskContext, task_timeline};
use codetether_provenance::ClaimProvenance;
/// Prepare a claimed task session in checkpoint and provenance order.
/// # Arguments
/// `host` supplies storage/workspaces; remaining inputs carry the task ID,
/// parsed context, timeline, accepted claim and provider credentials.
/// # Returns
/// The prepared native session and normalized agent type.
/// # Errors
/// Propagates loading, workspace resolution or Git preparation failures.
/// # Examples
/// ```no_run
/// # async fn prepare<H: codetether_a2a_worker_core::preparation::session::Host>(host: &H, context: &codetether_a2a_worker_core::task_config::TaskContext, timeline: &mut codetether_a2a_worker_core::task_timeline::TaskTimeline, claim: &codetether_provenance::ClaimProvenance) -> anyhow::Result<()> {
/// let _session = codetether_a2a_worker_core::preparation::session::prepare(host, "task", context, timeline, claim, None).await?;
/// # Ok(()) }
/// ```
pub async fn prepare<H: Host>(
    host: &H,
    task_id: &str,
    context: &TaskContext,
    timeline: &mut task_timeline::TaskTimeline,
    claim_provenance: &ClaimProvenance,
    provider_keys: Option<serde_json::Value>,
) -> anyhow::Result<(<H::Loader as Loader>::Session, String)> {
    let mut session = super::load_ids::<H::Loader>(
        context.resume_session_id.as_deref(),
        context.context_id.as_deref(),
        context.preserve_session_workspace,
    )
    .await?;
    timeline.checkpoint_with_detail(
        task_timeline::TaskCheckpoint::SessionReady,
        Some(format!("session_id={}", session.id())),
    );
    host.resolve_workspace(context, &mut session, timeline)
        .await?;
    let agent_type = crate::preparation::normalize_agent(&context.raw_agent);
    session.set_agent(agent_type.clone());
    session.attach_claim(claim_provenance);
    session.metadata_mut().provider_keys = provider_keys;
    if let Some(model) = context.selected_model.clone() {
        session.metadata_mut().model = Some(model);
    }
    if !context.is_virtual_task {
        crate::preparation::prepare_git(
            task_id,
            session.metadata().directory.as_deref(),
            session.metadata().provenance.as_ref(),
            context,
        )
        .await?;
        timeline.checkpoint(task_timeline::TaskCheckpoint::GitHookInstalled);
    }
    timeline.checkpoint_with_detail(
        task_timeline::TaskCheckpoint::ModelSelected,
        session.metadata().model.clone(),
    );
    Ok((session, agent_type))
}
