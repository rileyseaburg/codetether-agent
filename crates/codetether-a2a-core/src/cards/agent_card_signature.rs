use super::*;

/// JWS signature attached to an agent card
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// AgentCardSignature data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::AgentCardSignature { signature: String::new(), algorithm: None, key_id: None };
/// assert!(serde_json::to_value(&value).unwrap().is_object());
/// ```
pub struct AgentCardSignature {
    /// The JWS compact serialization payload
    pub signature: String,
    /// The algorithm used (e.g. "ES256", "RS256")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub algorithm: Option<String>,
    /// Key ID referencing the signing key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
}
