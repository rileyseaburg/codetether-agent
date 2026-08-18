//! Persona identity, policy, and runtime-state contracts.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Persona execution status.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::cognition::PersonaStatus;
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaIdentity {
    pub id: String,
    pub name: String,
    pub role: String,
    pub charter: String,
    pub swarm_id: Option<String>,
    pub parent_id: Option<String>,
    pub depth: u32,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub tags: Vec<String>,
}
