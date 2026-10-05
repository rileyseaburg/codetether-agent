//! Support helpers for the production LLM verifier.

use super::LlmVerifier;
use crate::provider::Provider;
use crate::tool::ToolRegistry;
use std::sync::Arc;

impl LlmVerifier {
    /// Verification-capability tools rooted at the verifier's workspace.
    pub(super) fn verification_tools(
        &self,
        provider: &Arc<dyn Provider>,
        model: &str,
    ) -> Arc<ToolRegistry> {
        crate::tool::swarm_execute::agent_registry::standard(
            false,
            true,
            &self.workspace,
            Arc::clone(provider),
            model.to_string(),
        )
    }

    /// Model recorded as the verifier in the verdict log.
    ///
    /// Uses the same precedence as
    /// [`resolve_verifier_model`](super::resolve_verifier_model) (without
    /// repeating its self-review warning), so the log names the model that
    /// actually ran.
    pub(super) async fn verdict_identity(&self) -> String {
        let chosen = super::selected_verifier_model()
            .or_else(|| std::env::var(super::VERIFIER_MODEL_ENV).ok());
        let default = crate::config::Config::load()
            .await
            .ok()
            .and_then(|config| config.default_model);
        super::select_verifier_model(
            chosen.as_deref(),
            self.worker_model.as_deref(),
            default.as_deref(),
        )
        .unwrap_or_else(|| "llm-verifier".into())
    }
}
