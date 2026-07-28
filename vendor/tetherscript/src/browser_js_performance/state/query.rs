//! Read and clear user-timing entries.

use super::*;

pub(super) fn entries() -> Vec<PerformanceEntry> {
    store::with(|timeline| timeline.entries.clone())
}

pub(super) fn by_type(entry_type: &str) -> Vec<PerformanceEntry> {
    store::with(|timeline| {
        timeline
            .entries
            .iter()
            .filter(|entry| entry.entry_type == entry_type)
            .cloned()
            .collect()
    })
}

pub(super) fn by_name(name: &str, entry_type: Option<&str>) -> Vec<PerformanceEntry> {
    store::with(|timeline| {
        timeline
            .entries
            .iter()
            .filter(|entry| entry.name == name)
            .filter(|entry| entry_type.is_none_or(|kind| entry.entry_type == kind))
            .cloned()
            .collect()
    })
}

pub(super) fn clear(entry_type: &str, name: Option<&str>) {
    store::with_mut(|timeline| {
        timeline.entries.retain(|entry| {
            entry.entry_type != entry_type || name.is_some_and(|name| entry.name != name)
        });
    });
}
