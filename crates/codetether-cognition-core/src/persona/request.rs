//! Request payloads for persona lifecycle operations.

use serde::{Deserialize, Serialize};

use super::PersonaPolicy;

/// Request payload for creating a persona.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// let now = chrono::Utc::now();
/// let value = CreatePersonaRequest { persona_id: None, name: "Reviewer".into(), role: "reviewer".into(), charter: "Review changes".into(), swarm_id: None, parent_id: None, policy: None, tags: vec![] };
/// assert!(value.persona_id.is_none());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePersonaRequest {
    /// Persona associated with this record or request.
    pub persona_id: Option<String>,
    /// Human-readable persona name.
    pub name: String,
    /// Role used for persona behavior and governance.
    pub role: String,
    /// Instructions defining the persona responsibility.
    pub charter: String,
    /// Optional owning swarm identifier.
    pub swarm_id: Option<String>,
    /// Optional parent persona identifier.
    pub parent_id: Option<String>,
    /// Resource and lineage limits.
    pub policy: Option<PersonaPolicy>,
    #[serde(default)]
    /// Persona classification tags.
    pub tags: Vec<String>,
}

/// Request payload for spawning a child persona.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// let now = chrono::Utc::now();
/// let value = SpawnPersonaRequest { persona_id: None, name: "Child".into(), role: "reviewer".into(), charter: "Review changes".into(), swarm_id: None, policy: None };
/// assert!(value.policy.is_none());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnPersonaRequest {
    /// Persona associated with this record or request.
    pub persona_id: Option<String>,
    /// Human-readable persona name.
    pub name: String,
    /// Role used for persona behavior and governance.
    pub role: String,
    /// Instructions defining the persona responsibility.
    pub charter: String,
    /// Optional owning swarm identifier.
    pub swarm_id: Option<String>,
    /// Resource and lineage limits.
    pub policy: Option<PersonaPolicy>,
}

/// Request payload for reaping persona(s).
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// let now = chrono::Utc::now();
/// let value = ReapPersonaRequest { cascade: Some(false), reason: Some("finished".into()) };
/// assert_eq!(value.cascade, Some(false));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReapPersonaRequest {
    /// Whether descendants should also be reaped.
    pub cascade: Option<bool>,
    /// Optional explanation for reaping.
    pub reason: Option<String>,
}
