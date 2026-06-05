//! FS directory scanning + sorting for workspace snapshots.
//!
//! This module contains the filesystem-facing portion of workspace snapshot
//! creation. It reads the immediate entries under a workspace root, filters out
//! names that should not appear in the sidebar, classifies each visible entry as
//! a file or directory, and sorts the resulting list for predictable display.

use std::path::Path;

use super::workspace_helpers::should_skip_entry;
use super::workspace_types::{WorkspaceEntry, WorkspaceEntryKind};

/// Collects visible workspace entries from a directory.
///
/// The scan is shallow: only direct children of `root` are returned. Entries
/// whose names are filtered by [`should_skip_entry`] are omitted. Directory
/// entries are classified with `std::fs::DirEntry::file_type`; entries whose
/// type cannot be read are treated as files so they can still appear in the
/// sidebar.
///
/// # Parameters
///
/// * `root` - Directory whose immediate children should be listed.
///
/// # Returns
///
/// Returns a vector of collected entries. If `root` cannot be read, an empty
/// vector is returned.
///
/// # Side Effects
///
/// Reads the filesystem metadata for entries under `root`.
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

/// Sorts workspace entries for stable sidebar display.
///
/// Directories are ordered before files. Entries within the same kind are sorted
/// by ASCII case-insensitive filename comparison without allocating temporary
/// lowercase strings.
///
/// # Parameters
///
/// * `entries` - Mutable slice of workspace entries to sort in place.
///
/// # Side Effects
///
/// Reorders `entries`.
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
