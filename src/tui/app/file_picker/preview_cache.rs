use super::preview::load_preview;
use super::types::FilePickerState;

pub fn refresh_preview(state: &mut FilePickerState) {
    state.preview = state
        .entries
        .get(state.selected)
        .map(|entry| load_preview(&entry.path));
    state.preview_scroll = 0;
}
