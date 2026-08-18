//! Child persona spawning under an existing parent.

use anyhow::Result;

use super::{CognitionRuntime, CreatePersonaRequest, PersonaRuntimeState, SpawnPersonaRequest};

impl CognitionRuntime {
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
        self.create_persona(CreatePersonaRequest {
            persona_id: req.persona_id,
            name: req.name,
            role: req.role,
            charter: req.charter,
            swarm_id: req.swarm_id,
            parent_id: Some(parent_id.to_string()),
            policy: req.policy,
            tags: Vec::new(),
        })
        .await
    }
}
