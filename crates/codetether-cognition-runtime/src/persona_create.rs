//! Native facade for shared cognition create behavior.
use super::{CognitionRuntime, CreatePersonaRequest, PersonaRuntimeState};
use crate::Host;
use anyhow::Result;
impl<H: Host> CognitionRuntime<H> {
    /// Create a persona record, inheriting lineage from its parent if any.
    ///
    /// # Errors
    ///
    /// Returns an error when the parent is invalid, a branching or depth limit
    /// is exceeded, or the requested persona ID already exists.
    pub async fn create_persona(&self, req: CreatePersonaRequest) -> Result<PersonaRuntimeState> {
        self.state.create_persona(req).await
    }
}
