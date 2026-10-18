//! Tracked delivery states and timestamped transition records.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Lifecycle state for a mailbox message.
///
/// # Examples
/// ```
/// use codetether_actor::DeliveryState;
/// let state = DeliveryState::Pending;
/// match state {
///     DeliveryState::Pending | DeliveryState::Delivered => {},
///     DeliveryState::Acked | DeliveryState::DeadLettered => {},
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeliveryState {
    /// Waiting for delivery.
    Pending,
    /// Removed from the destination mailbox.
    Delivered,
    /// Acknowledged by the receiving actor.
    Acked,
    /// Sent to dead-letter handling.
    DeadLettered,
}

/// Tracks mailbox delivery and acknowledgement metadata.
///
/// # Examples
/// ```
/// let record = codetether_actor::DeliveryRecord::pending();
/// assert_eq!(record.state, codetether_actor::DeliveryState::Pending);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryRecord {
    /// Current delivery state.
    pub state: DeliveryState,
    /// Recorded delivery attempts.
    pub attempts: u32,
    /// UTC time of the last transition.
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
