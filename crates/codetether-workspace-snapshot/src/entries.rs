//! Shallow workspace enumeration with shared pruning rules.
use crate::{WorkspaceEntry, WorkspaceEntryKind, should_skip_entry};
use std::path::Path;

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
/// # Examples
/// ```no_run
/// let _entries = codetether_workspace_snapshot::collect_entries(std::path::Path::new("."));
/// ```
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
