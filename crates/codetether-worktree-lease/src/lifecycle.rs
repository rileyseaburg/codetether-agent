//! Renewal, release, and inspection of mux-owned leases.

use super::{CoordinationReply, LeaseRegistry, time};

impl LeaseRegistry {
    /// Renew unexpired leases for `owner`; panics if the table mutex is poisoned.
    pub fn renew(&self, owner: &str) -> CoordinationReply {
        let mut entries = self.entries.lock().unwrap();
        reap(&mut entries);
        let mut count = 0;
        for lease in entries.values_mut().filter(|lease| lease.owner == owner) {
            lease.expires_at_ms = time::expiry();
            count += 1;
        }
        CoordinationReply::Renewed { count }
    }

    /// Remove `owner` leases and wake waiters; panics if the mutex is poisoned.
    pub fn release(&self, owner: &str) -> CoordinationReply {
        let count = {
            let mut entries = self.entries.lock().unwrap();
            let before = entries.len();
            entries.retain(|_, lease| lease.owner != owner);
            before - entries.len()
        };
        if count > 0 {
            self.signal();
        }
        CoordinationReply::Released { count }
    }

    /// Snapshot unexpired leases; panics if the table mutex is poisoned.
    pub fn snapshot(&self) -> CoordinationReply {
        let mut entries = self.entries.lock().unwrap();
        reap(&mut entries);
        CoordinationReply::Snapshot {
            leases: entries.values().cloned().collect(),
        }
    }
}

fn reap(entries: &mut std::collections::HashMap<super::key::LeaseKey, super::WorktreeLease>) {
    entries.retain(|_, lease| lease.expires_at_ms > time::now_ms());
}
