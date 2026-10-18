//! Child persona spawning under an existing parent.

use crate::Host;
use anyhow::Result;

use super::{CognitionRuntime, PersonaRuntimeState, SpawnPersonaRequest};

impl<H: Host> CognitionRuntime<H> {
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
        self.state.spawn_child(parent_id, req).await
    }
}
