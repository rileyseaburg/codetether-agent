//! Resolve and record actual provider routing before executing a verifier.

use super::{
    LlmVerifier, VerificationRequest, observation::Ticket, prompt, resolve_verifier_model,
};
use crate::swarm::executor::{AgentLoopExit, run_agent_loop};

pub(super) async fn run(
    verifier: &LlmVerifier,
    request: &VerificationRequest,
    ticket: &mut Ticket,
) -> anyhow::Result<String> {
    let requested = resolve_verifier_model(verifier.worker_model.as_deref()).await?;
    ticket.requested(requested.clone());
    let providers = crate::provider::ProviderRegistry::shared_from_vault().await?;
    let (provider, model) = providers.resolve_model(&requested)?;
    ticket.resolved(provider.name(), &model);
    let tools = verifier.verification_tools(&provider, &model);
    let id = ticket.id().to_string();
    tracing::info!(verifier = %id, provider = %provider.name(), model = %model,
        requested_model = %requested, claimed = request.claimed.as_str(), "Starting goal verifier");
    let system = prompt::system_prompt(&verifier.workspace, &model, request.claimed);
    let user = prompt::user_prompt(request);
    match run_agent_loop(
        provider,
        &model,
        &system,
        &user,
        tools.definitions(),
        tools,
        40,
        600,
        None,
        id,
        None,
        Some(verifier.workspace.clone()),
    )
    .await?
    {
        (report, _, _, AgentLoopExit::Completed) => Ok(report),
        (_, _, _, exit) => anyhow::bail!("verifier stopped before a verdict: {exit:?}"),
    }
}
