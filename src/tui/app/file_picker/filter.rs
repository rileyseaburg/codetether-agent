use super::scan::scan_directory;
use super::types::FilePickerState;

pub fn rescan_with_filter(state: &mut FilePickerState) {
    let filter = state.filter.to_ascii_lowercase();
    let entries = scan_directory(&state.dir);
    state.entries = if filter.is_empty() {
        entries
    } else {
        entries
            .into_iter()
            .filter(|e| e.name.to_ascii_lowercase().contains(&filter) || e.name == "../")
            .collect()
    };
    state.selected = 0;
    super::preview_cache::refresh_preview(state);
}
