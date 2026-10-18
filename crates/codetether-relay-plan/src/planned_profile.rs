//! Deserializable planned profile.
use serde::Deserialize;

/// Planner-supplied specialist fields before normalization.
/// # Examples
/// ```
/// let profile: codetether_relay_plan::PlannedRelayProfile = serde_json::from_str("{}").unwrap();
/// assert!(profile.name.is_empty());
/// ```
#[derive(Debug, Clone, Deserialize)]
pub struct PlannedRelayProfile {
    #[serde(default)]
    /// Specialist name; blank planned names receive a generated fallback.
    pub name: String,
    #[serde(default)]
    /// Specialty description used for instructions and capability labels.
    pub specialty: String,
    #[serde(default)]
    /// Specialist mission; blank values receive a fallback.
    pub mission: String,
    #[serde(default)]
    /// Capability labels associated with this specialist.
    pub capabilities: Vec<String>,
}
