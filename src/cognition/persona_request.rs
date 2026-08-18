//! Request payloads for persona lifecycle operations.

use serde::{Deserialize, Serialize};

use super::PersonaPolicy;

/// Request payload for creating a persona.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePersonaRequest {
    pub persona_id: Option<String>,
    pub name: String,
    pub role: String,
    pub charter: String,
    pub swarm_id: Option<String>,
    pub parent_id: Option<String>,
    pub policy: Option<PersonaPolicy>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Request payload for spawning a child persona.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnPersonaRequest {
    pub persona_id: Option<String>,
    pub name: String,
    pub role: String,
    pub charter: String,
    pub swarm_id: Option<String>,
    pub policy: Option<PersonaPolicy>,
}

/// Request payload for reaping persona(s).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReapPersonaRequest {
    pub cascade: Option<bool>,
    pub reason: Option<String>,
}
