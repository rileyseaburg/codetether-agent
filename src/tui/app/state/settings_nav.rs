//! Settings UI navigation methods.

use crate::tui::models::ViewMode;

impl super::AppState {
    pub(crate) const SETTINGS_COUNT: usize = 4;

    pub fn settings_select_prev(&mut self) {
        if self.selected_settings_index > 0 {
            self.selected_settings_index -= 1;
        }
    }

    pub fn settings_select_next(&mut self) {
        if self.selected_settings_index + 1 < Self::SETTINGS_COUNT {
            self.selected_settings_index += 1;
        }
    }

    pub fn set_view_mode(&mut self, view_mode: ViewMode) {
        if self.view_mode == ViewMode::Chat && view_mode != ViewMode::Chat {
            self.save_scroll_state();
        }
        self.view_mode = view_mode;
    }
}
