use super::*;

/// A security requirement entry: name → list of required scopes
/// SecurityRequirement data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let requirements: codetether_a2a_core::SecurityRequirement = Default::default();
/// assert!(requirements.is_empty());
/// ```
pub type SecurityRequirement = HashMap<String, Vec<String>>;
