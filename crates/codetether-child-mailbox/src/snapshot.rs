//! Versioned mailbox snapshot and crash recovery rules.

use super::item::{DeliveryState, Item};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Versioned durable queue for one child.
///
/// # Examples
/// ```
/// let snapshot=codetether_child_mailbox::Snapshot::new("child");
/// assert!(snapshot.items.is_empty());
/// ```
#[derive(Clone, Deserialize, Serialize)]
pub struct Snapshot {
    /// Persistence format version.
    pub version: u8,
    /// Owning durable child identity.
    pub child_session_id: String,
    /// FIFO deliveries including any claimed front item.
    pub items: VecDeque<Item>,
}

impl Snapshot {
    /// Construct an empty version-one snapshot.
    pub fn new(child_session_id: &str) -> Self {
        Self {
            version: 1,
            child_session_id: child_session_id.into(),
            items: VecDeque::new(),
        }
    }

    /// Reset claimed deliveries to pending after process restart.
    pub fn recover(&mut self) -> bool {
        let mut changed = self.version != 1;
        self.version = 1;
        for item in &mut self.items {
            if item.state == DeliveryState::Running {
                item.state = DeliveryState::Pending;
                changed = true;
            }
        }
        changed
    }
}

#[cfg(test)]
#[path = "snapshot_tests.rs"]
mod tests;
