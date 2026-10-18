//! Settings UI navigation methods.

use crate::tui::models::ViewMode;

impl super::AppState {
    pub(crate) const SETTINGS_COUNT: usize = 9;

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
        codetether_tui_navigation::view_state::set_view_mode(
            &mut self.view_mode,
            view_mode,
            &mut self.saved_scroll,
            self.chat_viewport.chat_scroll,
            self.chat_viewport.chat_auto_follow,
            self.tool_viewport.tool_preview_scroll,
        );
    }
}
