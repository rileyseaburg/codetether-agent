//! FS directory scanning + sorting for workspace snapshots.

use std::path::Path;

use super::workspace_helpers::should_skip_entry;
use super::workspace_types::{WorkspaceEntry, WorkspaceEntryKind};

pub fn collect_entries(root: &Path) -> Vec<WorkspaceEntry> {
    let Ok(rd) = std::fs::read_dir(root) else {
        return Vec::new();
    };
    rd.flatten()
        .filter_map(|entry| {
            // `into_string()` is a zero-copy conversion when the name is
            // valid UTF-8 (the common case on every platform we support);
            // fall back to `to_string_lossy` only when it isn't.
            let name = entry
                .file_name()
                .into_string()
                .unwrap_or_else(|os| os.to_string_lossy().into_owned());
            if should_skip_entry(&name) {
                return None;
            }
            let kind = match entry.file_type() {
                Ok(ft) if ft.is_dir() => WorkspaceEntryKind::Directory,
                _ => WorkspaceEntryKind::File,
            };
            Some(WorkspaceEntry { name, kind })
        })
        .collect()
}

pub fn sort_entries(entries: &mut [WorkspaceEntry]) {
    // Case-insensitive compare WITHOUT allocating a lowercased String
    // per comparison. On a workspace with many entries the old code
    // produced O(n log n) allocations of filename-sized strings; this
    // does zero allocations regardless of entry count.
    fn cmp_ascii_ci(a: &str, b: &str) -> std::cmp::Ordering {
        let ab = a.as_bytes();
        let bb = b.as_bytes();
        let len = ab.len().min(bb.len());
        for i in 0..len {
            let av = ab[i].to_ascii_lowercase();
            let bv = bb[i].to_ascii_lowercase();
            if av != bv {
                return av.cmp(&bv);
            }
        }
        ab.len().cmp(&bb.len())
    }
    entries.sort_by(|a, b| match (a.kind, b.kind) {
        (WorkspaceEntryKind::Directory, WorkspaceEntryKind::File) => std::cmp::Ordering::Less,
        (WorkspaceEntryKind::File, WorkspaceEntryKind::Directory) => std::cmp::Ordering::Greater,
        _ => cmp_ascii_ci(&a.name, &b.name),
    });
}
