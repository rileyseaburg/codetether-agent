//! Atomic multi-path acquisition for the mux lease table.

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
        if paths
            .iter()
            .any(|path| super::workspace_claim(workspace, path))
        {
            return CoordinationReply::WorkspaceScopeForbidden;
        }
        let mut entries = self.entries.lock().unwrap();
        entries.retain(|_, lease| lease.expires_at_ms > time::now_ms());
        if let Some(reply) = super::conflict::blocked(&entries, owner, workspace, &paths) {
            return reply;
        }
        let leases = paths
            .into_iter()
            .map(|path| super::claim::insert(&mut entries, owner, agent, workspace, path))
            .collect();
        CoordinationReply::Acquired {
            leases,
            waited_ms: 0,
        }
    }
}
