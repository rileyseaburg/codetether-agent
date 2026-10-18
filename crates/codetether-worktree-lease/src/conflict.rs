//! Conflict detection while the caller holds the atomic lease-table lock.

use super::{
    CoordinationReply, WorktreeLease,
    key::{LeaseKey, overlaps},
    time,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub(super) fn blocked(
    entries: &HashMap<LeaseKey, WorktreeLease>,
    owner: &str,
    workspace: &Path,
    paths: &[PathBuf],
) -> Option<CoordinationReply> {
    let conflicts = entries
        .values()
        .filter(|lease| {
            lease.workspace == workspace
                && lease.owner != owner
                && paths.iter().any(|path| overlaps(&lease.path, path))
        })
        .cloned()
        .collect::<Vec<_>>();
    if conflicts.is_empty() {
        return None;
    }
    let retry_after_ms = conflicts
        .iter()
        .map(|lease| time::remaining(lease.expires_at_ms))
        .min()
        .unwrap_or_default();
    Some(CoordinationReply::Blocked {
        conflicts,
        waited_ms: 0,
        retry_after_ms,
    })
}
