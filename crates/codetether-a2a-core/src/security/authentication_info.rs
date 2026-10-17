use super::*;

/// Authentication info for connecting to an agent (used by clients)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// AuthenticationInfo data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::AuthenticationInfo { schemes: vec![], credentials: None };
/// assert!(serde_json::to_value(&value).unwrap().is_object());
/// ```
pub struct AuthenticationInfo {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub schemes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<String>,
}
