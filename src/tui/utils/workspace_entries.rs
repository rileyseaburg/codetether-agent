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
            let name = entry.file_name().to_string_lossy().to_string();
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
    entries.sort_by(|a, b| match (a.kind, b.kind) {
        (WorkspaceEntryKind::Directory, WorkspaceEntryKind::File) => std::cmp::Ordering::Less,
        (WorkspaceEntryKind::File, WorkspaceEntryKind::Directory) => std::cmp::Ordering::Greater,
        _ => a
            .name
            .to_ascii_lowercase()
            .cmp(&b.name.to_ascii_lowercase()),
    });
}
