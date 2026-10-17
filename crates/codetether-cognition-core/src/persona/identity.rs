//! Persona identity, policy, and runtime-state contracts.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Persona execution status.
///
/// # Examples
///
/// ```rust
/// use codetether_cognition_core::PersonaStatus;
/// assert_eq!(PersonaStatus::Active, PersonaStatus::Active);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersonaStatus {
    Active,
    Idle,
    Reaped,
    Error,
}

/// Identity contract for a persona.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// let now = chrono::Utc::now();
/// let value = PersonaIdentity { id: "p1".into(), name: "Reviewer".into(), role: "reviewer".into(), charter: "Review changes".into(), swarm_id: None, parent_id: None, depth: 0, created_at: chrono::Utc::now(), tags: vec![] };
/// assert_eq!(value.depth, 0);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaIdentity {
    /// Stable record identifier.
    pub id: String,
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
    /// Depth in the persona lineage.
    pub depth: u32,
    /// Time the record was created.
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    /// Persona classification tags.
    pub tags: Vec<String>,
}
