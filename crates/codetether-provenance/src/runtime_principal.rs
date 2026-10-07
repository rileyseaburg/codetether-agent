//! Display-safe runtime principal metadata for session and mux projections.

use serde::{Deserialize, Serialize};

use super::ExecutionProvenance;

/// Identity metadata attached to runtime state rather than conversational text.
///
/// # Examples
///
/// ```
/// use codetether_provenance::RuntimePrincipal;
///
/// let principal = RuntimePrincipal::default();
/// assert!(principal.agent_identity_id.is_none());
/// ```
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct RuntimePrincipal {
    /// Human-facing agent or persona name.
    pub agent_name: String,
    /// Stable CodeTether/Forgejo agent identity.
    pub agent_identity_id: Option<String>,
    /// Persona used to provision the workload identity.
    pub persona_id: Option<String>,
    /// SPIFFE workload identifier asserted by the runtime.
    pub spiffe_id: Option<String>,
    /// Provenance record for the active durable session.
    pub provenance_id: Option<String>,
}

impl RuntimePrincipal {
    /// Build the principal for a session's agent and provenance.
    pub fn for_session(agent_name: &str, provenance: Option<&ExecutionProvenance>) -> Self {
        Self {
            agent_name: agent_name.to_string(),
            agent_identity_id: provenance
                .and_then(|item| item.identity.agent_identity_id.clone())
                .or_else(super::runtime_agent_identity),
            persona_id: runtime_persona_id(),
            spiffe_id: runtime_spiffe_id(),
            provenance_id: provenance.map(|item| item.provenance_id.clone()),
        }
    }
}

/// Persona ID from `CODETETHER_PERSONA_ID`, when set.
pub fn runtime_persona_id() -> Option<String> {
    environment("CODETETHER_PERSONA_ID")
}

/// SPIFFE ID from `CODETETHER_SPIFFE_ID` or `SPIFFE_ID`, when set.
pub fn runtime_spiffe_id() -> Option<String> {
    environment("CODETETHER_SPIFFE_ID").or_else(|| environment("SPIFFE_ID"))
}

fn environment(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}
