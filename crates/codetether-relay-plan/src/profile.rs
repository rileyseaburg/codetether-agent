//! Resolved relay profile contract.

/// Resolved specialist name, instructions and capabilities.
/// # Examples
/// ```
/// let profile = codetether_relay_plan::RelayProfile { name: "auto-review".into(), instructions: "Review changes".into(), capabilities: vec!["review".into()] };
/// assert_eq!(profile.name, "auto-review");
/// ```
#[derive(Debug, Clone)]
pub struct RelayProfile {
    /// Specialist name; blank planned names receive a generated fallback.
    pub name: String,
    /// Instructions used by the runtime specialist.
    pub instructions: String,
    /// Capability labels associated with this specialist.
    pub capabilities: Vec<String>,
}
