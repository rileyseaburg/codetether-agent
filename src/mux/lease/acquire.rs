//! Atomic multi-path acquisition for the mux lease table.

use super::key::overlaps;
use super::{CoordinationReply, LeaseRegistry, time};
use std::path::{Path, PathBuf};

impl LeaseRegistry {
    pub(in crate::mux) fn acquire(
        &self,
        owner: &str,
        agent: &str,
        workspace: &Path,
        paths: Vec<PathBuf>,
    ) -> CoordinationReply {
        let mut entries = self.entries.lock().unwrap();
        entries.retain(|_, lease| lease.expires_at_ms > time::now_ms());
        let conflicts = entries
            .values()
            .filter(|lease| {
                lease.workspace == workspace
                    && lease.owner != owner
                    && paths.iter().any(|path| overlaps(&lease.path, path))
            })
            .cloned()
            .collect::<Vec<_>>();
        if !conflicts.is_empty() {
            return CoordinationReply::Blocked { conflicts };
        }
        let leases = paths
            .into_iter()
            .map(|path| super::claim::insert(&mut entries, owner, agent, workspace, path))
            .collect();
        CoordinationReply::Acquired { leases }
    }
}
