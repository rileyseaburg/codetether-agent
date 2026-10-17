use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// OAuthFlowAuthorizationCode data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::OAuthFlowAuthorizationCode { authorization_url: String::new(), token_url: String::new(), refresh_url: None, scopes: std::collections::HashMap::new() };
/// assert!(serde_json::to_value(&value).unwrap().is_object());
/// ```
pub struct OAuthFlowAuthorizationCode {
    pub authorization_url: String,
    pub token_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    #[serde(default)]
    pub scopes: HashMap<String, String>,
}
