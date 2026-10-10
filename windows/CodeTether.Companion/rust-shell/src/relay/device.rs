use chrono::{DateTime, Utc};
use zeroize::Zeroizing;

/// Memory-only device capability. It deliberately implements neither `Debug` nor `Clone`.
pub(crate) struct Device {
    pub(super) agent: reqwest::Client,
    pub(super) session_id: String,
    pub(super) token: Zeroizing<String>,
    pub(crate) interval_seconds: u16,
    pub(crate) expires_at: DateTime<Utc>,
    /// Attempted reply UUIDs survive worker restarts; never retains reply text.
    pub(super) replies: std::sync::Mutex<std::collections::HashSet<uuid::Uuid>>,
}

impl Device {
    pub(super) fn path(&self, action: &str) -> String {
        format!(
            "{}/sessions/{}/{}",
            super::client::ORIGIN,
            self.session_id,
            action
        )
    }

    /// Returns whether the relay-issued session deadline has elapsed.
    pub(crate) fn expired(&self) -> bool {
        Utc::now() >= self.expires_at
    }
}
