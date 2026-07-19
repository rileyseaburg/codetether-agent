//! Insertion of one validated path claim into the lease table.

use super::key::LeaseKey;
use super::{WorktreeLease, time};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub(super) fn insert(
    entries: &mut HashMap<LeaseKey, WorktreeLease>,
    owner: &str,
    agent: &str,
    workspace: &Path,
    path: PathBuf,
) -> WorktreeLease {
    let lease = WorktreeLease {
        owner: owner.into(),
        agent: agent.into(),
        workspace: workspace.into(),
        path: path.clone(),
        expires_at_ms: time::expiry(),
    };
    let key = LeaseKey {
        workspace: workspace.into(),
        path,
    };
    entries.insert(key, lease.clone());
    lease
}
