//! Stable directory-first workspace display ordering.
use crate::{WorkspaceEntry, WorkspaceEntryKind};

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
/// # Returns
/// Unit after ordering the supplied entries.
/// # Examples
/// ```
/// let mut entries = Vec::new();
/// codetether_workspace_snapshot::sort_entries(&mut entries);
/// assert!(entries.is_empty());
/// ```
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
