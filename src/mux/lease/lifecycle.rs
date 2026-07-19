//! Renewal, release, and inspection of mux-owned leases.

use super::{CoordinationReply, LeaseRegistry, time};

impl LeaseRegistry {
    pub(in crate::mux) fn renew(&self, owner: &str) -> CoordinationReply {
        let mut entries = self.entries.lock().unwrap();
        reap(&mut entries);
        let mut count = 0;
        for lease in entries.values_mut().filter(|lease| lease.owner == owner) {
            lease.expires_at_ms = time::expiry();
            count += 1;
        }
        CoordinationReply::Renewed { count }
    }

    pub(in crate::mux) fn release(&self, owner: &str) -> CoordinationReply {
        let mut entries = self.entries.lock().unwrap();
        let before = entries.len();
        entries.retain(|_, lease| lease.owner != owner);
        CoordinationReply::Released {
            count: before - entries.len(),
        }
    }

    pub(in crate::mux) fn snapshot(&self) -> CoordinationReply {
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
