//! Resolve provider identity before executing a bounded verification run.
use super::{Host, Run};
use crate::VerificationRequest;
use codetether_swarm_core::agent_retry::AgentLoopExit;
use codetether_verifier_state::{observation::Ticket, resolve_verifier_model};
use std::path::Path;
pub(super) async fn run<H: Host>(
    worker_model: Option<&str>,
    workspace: &Path,
    request: &VerificationRequest,
    ticket: &mut Ticket,
) -> anyhow::Result<String> {
    let requested = resolve_verifier_model(worker_model).await?;
    ticket.requested(requested.clone());
    let (_providers, provider, model) = H::resolve(&requested).await?;
    let resolved = provider.resolved_model_identity(&model);
    ticket.resolved(provider.name(), &resolved);
    let tools = H::tools(workspace, &provider, &model);
    let id = ticket.id().to_string();
    tracing::info!(verifier = %id, provider = %provider.name(), model = %resolved,
        requested_model = %requested, claimed = request.claimed.as_str(), "Starting goal verifier");
    let system = H::system_prompt(workspace, &model, request.claimed);
    let user = crate::user_prompt(request);
    match H::execute(Run {
        provider,
        model: &model,
        system: &system,
        user: &user,
        definitions: H::definitions(&tools),
        tools,
        max_steps: 40,
        timeout_secs: 600,
        id,
        workspace: workspace.to_path_buf(),
    })
    .await?
    {
        (report, _, _, AgentLoopExit::Completed) => Ok(report),
        (_, _, _, exit) => anyhow::bail!("verifier stopped before a verdict: {exit:?}"),
    }
}
