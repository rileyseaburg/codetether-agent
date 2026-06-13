//! Drop index entries whose backing session file is gone.

use std::collections::{HashMap, HashSet};

use super::summary::SessionSummary;

/// Retain only entries whose id was seen on disk by the directory walk
/// ([`super::disk_walk::collect_disk_summaries`]). Pure in-memory set
/// lookups — no per-entry stat calls.
pub(super) fn retain_existing(
    index: &mut HashMap<String, SessionSummary>,
    existing_ids: &HashSet<String>,
) {
    index.retain(|id, _| existing_ids.contains(id));
}
