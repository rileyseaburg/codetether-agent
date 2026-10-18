//! Race-free change notification for queued lease acquisition requests.

use super::LeaseRegistry;

impl LeaseRegistry {
    pub(super) fn subscribe(&self) -> tokio::sync::watch::Receiver<u64> {
        self.changes.subscribe()
    }

    pub(super) fn signal(&self) {
        self.changes
            .send_modify(|version| *version = version.wrapping_add(1));
    }
}
