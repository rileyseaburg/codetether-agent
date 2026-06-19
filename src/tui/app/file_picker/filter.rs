use super::types::{FilePickerEntry, FilePickerState};

/// Re-apply the current filter to the cached directory listing.
///
/// This never touches the disk: it filters `state.all_entries` (populated
/// once when the directory is opened or changed) so typing in the filter
/// box stays responsive on large directories.
pub fn rescan_with_filter(state: &mut FilePickerState) {
    let filter = state.filter.to_ascii_lowercase();
    state.entries = if filter.is_empty() {
        state.all_entries.clone()
    } else {
        state
            .all_entries
            .iter()
            .filter(|e| matches_filter(e, &filter))
            .cloned()
            .collect()
    };
    state.selected = 0;
    super::preview_cache::refresh_preview(state);
}

fn matches_filter(entry: &FilePickerEntry, filter: &str) -> bool {
    entry.name == "../" || entry.name.to_ascii_lowercase().contains(filter)
}
