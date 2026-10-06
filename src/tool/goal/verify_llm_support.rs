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
}
