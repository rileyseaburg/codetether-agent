//! Versioned mailbox snapshot and crash recovery rules.

use super::item::{DeliveryState, Item};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Clone, Deserialize, Serialize)]
pub(super) struct Snapshot {
    pub(super) version: u8,
    pub(super) child_session_id: String,
    pub(super) items: VecDeque<Item>,
}

impl Snapshot {
    pub(super) fn new(child_session_id: &str) -> Self {
        Self {
            version: 1,
            child_session_id: child_session_id.into(),
            items: VecDeque::new(),
        }
    }

    pub(super) fn recover(&mut self) -> bool {
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
