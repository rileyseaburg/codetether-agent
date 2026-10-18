use super::super::CognitionRuntime;
use crate::{PersonaRuntimeState, Proposal, beliefs::Belief};
use std::collections::HashMap;
impl<T, R> CognitionRuntime<T, R> {
    /// Get current beliefs.
    pub async fn get_beliefs(&self) -> HashMap<String, Belief> {
        self.beliefs.read().await.clone()
    }

    /// Get a single belief by ID.
    pub async fn get_belief(&self, id: &str) -> Option<Belief> {
        self.beliefs.read().await.get(id).cloned()
    }

    /// Get a single persona's runtime state by ID.
    pub async fn get_persona(&self, id: &str) -> Option<PersonaRuntimeState> {
        self.personas.read().await.get(id).cloned()
    }

    /// Get all proposals.
    pub async fn get_proposals(&self) -> HashMap<String, Proposal> {
        self.proposals.read().await.clone()
    }
}
