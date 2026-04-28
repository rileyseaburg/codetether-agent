use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Lifecycle state for a mailbox message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryState {
    Pending,
    Delivered,
    Acked,
    DeadLettered,
}

/// Tracks mailbox delivery and acknowledgement metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryRecord {
    pub state: DeliveryState,
    pub attempts: u32,
    pub updated_at: DateTime<Utc>,
}

impl DeliveryRecord {
    /// Creates a record for a newly queued message.
    pub fn pending() -> Self {
        Self {
            state: DeliveryState::Pending,
            attempts: 0,
            updated_at: Utc::now(),
        }
    }

    /// Moves the record to a new delivery state.
    pub fn mark(&mut self, state: DeliveryState) {
        self.state = state;
        self.updated_at = Utc::now();
    }
}
