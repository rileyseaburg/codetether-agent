//! Production verifier: a separate LLM agent loop over the workspace.

use super::{VerificationRequest, VerifierAgent, prompt, resolve_verifier_model};
use crate::swarm::executor::{AgentLoopExit, run_agent_loop};
use async_trait::async_trait;
use std::{path::PathBuf, sync::Arc};

const MAX_STEPS: usize = 40;
const TIMEOUT_SECS: u64 = 600;

/// Verifier that runs a second LLM in its own agent thread.
///
/// It gets read and verification tools (it can read files and run tests and
/// git) but cannot edit or commit, so it can only judge the work, not do it.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tool::goal::verify::LlmVerifier;
/// use std::path::PathBuf;
///
/// let verifier = LlmVerifier {
///     worker_model: Some("openai/gpt-5".into()),
///     workspace: PathBuf::from("/repo"),
/// };
/// assert_eq!(verifier.workspace, PathBuf::from("/repo"));
/// ```
#[derive(Clone, Debug)]
pub struct LlmVerifier {
    /// Model the worker ran on; used when no verifier model is selected.
    pub worker_model: Option<String>,
    /// Directory the verifier inspects.
    pub workspace: PathBuf,
}

#[async_trait]
impl VerifierAgent for LlmVerifier {
    async fn review(&self, request: &VerificationRequest) -> anyhow::Result<String> {
        let requested = resolve_verifier_model(self.worker_model.as_deref()).await?;
        let providers = crate::provider::ProviderRegistry::shared_from_vault().await?;
        let (provider, model) = providers.resolve_model(&requested)?;
        let tools = crate::tool::swarm_execute::agent_registry::standard(
            false,
            true,
            &self.workspace,
            Arc::clone(&provider),
            model.clone(),
        );
        let id = format!("goal-verifier-{}", uuid::Uuid::new_v4());
        tracing::info!(verifier = %id, model = %model, claimed = request.claimed.as_str(), "Starting goal verifier");
        let system = prompt::system_prompt(&self.workspace, &model, request.claimed);
        let user = prompt::user_prompt(request);
        let run = run_agent_loop(
            provider,
            &model,
            &system,
            &user,
            tools.definitions(),
            tools,
            MAX_STEPS,
            TIMEOUT_SECS,
            None,
            id,
            None,
            Some(self.workspace.clone()),
        );
        match run.await? {
            (report, _, _, AgentLoopExit::Completed) => Ok(report),
            (_, _, _, exit) => anyhow::bail!("verifier stopped before a verdict: {exit:?}"),
        }
    }
}
