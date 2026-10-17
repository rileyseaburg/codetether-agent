use super::*;

// ─── Push Notification CRUD Types ────────────────────────────────────────

/// Request to set push notification config for a task
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// TaskPushNotificationConfig data used by A2A protocol consumers.
///
/// # Examples
/// ```
/// let value = codetether_a2a_core::TaskPushNotificationConfig { id: String::new(), push_notification_config: codetether_a2a_core::PushNotificationConfig { url: String::new(), token: None, id: None } };
/// assert!(serde_json::to_value(&value).unwrap().is_object());
/// ```
pub struct TaskPushNotificationConfig {
    pub id: String, // task id
    pub push_notification_config: PushNotificationConfig,
}
