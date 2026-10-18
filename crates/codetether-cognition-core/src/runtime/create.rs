//! Persona creation and lifecycle-event publication on shared runtime state.
use super::CognitionRuntime;
use crate::persona::lifecycle::{create_persona, spawned_event};
use crate::{CreatePersonaRequest, PersonaRuntimeState};
use anyhow::Result;

impl<T, R> CognitionRuntime<T, R> {
    /// Create a persona record, inheriting lineage from its parent if any.
    ///
    /// # Errors
    ///
    /// Returns an error when the parent is invalid, a branching or depth limit
    /// is exceeded, or the requested persona ID already exists.
    pub async fn create_persona(&self, req: CreatePersonaRequest) -> Result<PersonaRuntimeState> {
        let (persona, now) = create_persona(&self.personas, &self.default_policy, req).await?;
        self.push_event(spawned_event(&persona, now)).await;
        Ok(persona)
    }
}
