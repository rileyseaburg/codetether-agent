//! Publish lifecycle events after the shared reap mutation releases its lock.
use super::{CognitionRuntime, ReapPersonaRequest, ReapPersonaResponse};
use anyhow::Result;

impl CognitionRuntime {
    /// Reap one persona or its full descendant tree.
    ///
    /// # Errors
    ///
    /// Returns an error when `persona_id` is unknown.
    pub async fn reap_persona(
        &self,
        persona_id: &str,
        req: ReapPersonaRequest,
    ) -> Result<ReapPersonaResponse> {
        self.state.reap_persona(persona_id, req).await
    }
}
