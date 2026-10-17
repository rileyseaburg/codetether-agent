//! Task push-notification configuration.
//!
//! These serializable contracts are exported from the crate root for transport consumers.
use super::*;
mod push_notification_config;
pub use push_notification_config::PushNotificationConfig;
mod task_push_notification_config;
pub use task_push_notification_config::TaskPushNotificationConfig;
