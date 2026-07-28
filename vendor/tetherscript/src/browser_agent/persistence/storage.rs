//! Storage map conversion helpers for native snapshots.

use std::collections::HashMap;

use super::types::StorageOriginSnapshot;

pub(crate) fn snapshot_storage(
    storage: &HashMap<String, HashMap<String, String>>,
) -> Vec<StorageOriginSnapshot> {
    let mut origins: Vec<_> = storage
        .iter()
        .map(|(origin, entries)| StorageOriginSnapshot {
            origin: origin.clone(),
            entries: sorted_entries(entries),
        })
        .collect();
    origins.sort_by(|left, right| left.origin.cmp(&right.origin));
    origins
}

pub(crate) fn restore_storage(
    snapshots: &[StorageOriginSnapshot],
) -> HashMap<String, HashMap<String, String>> {
    snapshots
        .iter()
        .map(|bucket| {
            (
                bucket.origin.clone(),
                bucket.entries.iter().cloned().collect(),
            )
        })
        .collect()
}

fn sorted_entries(entries: &HashMap<String, String>) -> Vec<(String, String)> {
    let mut pairs: Vec<_> = entries
        .iter()
        .map(|(key, value)| (key.clone(), value.clone()))
        .collect();
    pairs.sort_by(|left, right| left.0.cmp(&right.0));
    pairs
}
