//! Child persona spawning under an existing parent.

use anyhow::Result;

use super::CognitionRuntime;
use crate::{PersonaRuntimeState, SpawnPersonaRequest};

impl<T, R> CognitionRuntime<T, R> {
    /// Spawn a child persona under an existing parent.
    ///
    /// # Errors
    ///
    /// Propagates the lineage and policy errors from
    /// [`create_persona`](CognitionRuntime::create_persona).
    pub async fn spawn_child(
        &self,
        parent_id: &str,
        req: SpawnPersonaRequest,
    ) -> Result<PersonaRuntimeState> {
        self.create_persona(crate::persona::lifecycle::child_request(parent_id, req))
            .await
    }
}
