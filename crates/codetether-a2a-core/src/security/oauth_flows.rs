use super::*;

/// OAuth 2.0 flows (matches proto `OAuthFlows`)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// OAuthFlows data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::OAuthFlows { implicit: None, authorization_code: None, client_credentials: None, device_code: None };
/// assert!(serde_json::to_value(&value).unwrap().is_object());
/// ```
pub struct OAuthFlows {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit: Option<OAuthFlowImplicit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<OAuthFlowAuthorizationCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials: Option<OAuthFlowClientCredentials>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_code: Option<OAuthFlowDeviceCode>,
}
