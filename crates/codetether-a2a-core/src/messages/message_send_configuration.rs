use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// MessageSendConfiguration data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::MessageSendConfiguration { accepted_output_modes: vec![], blocking: None, history_length: None, push_notification_config: None };
/// assert!(serde_json::to_value(&value).unwrap().is_object());
/// ```
pub struct MessageSendConfiguration {
    #[serde(default)]
    pub accepted_output_modes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_notification_config: Option<PushNotificationConfig>,
}
