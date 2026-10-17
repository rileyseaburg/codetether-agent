use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// PushNotificationConfig data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::PushNotificationConfig { url: String::new(), token: None, id: None };
/// assert!(serde_json::to_value(&value).unwrap().is_object());
/// ```
pub struct PushNotificationConfig {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
