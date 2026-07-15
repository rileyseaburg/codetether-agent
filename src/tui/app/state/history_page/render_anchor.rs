//! Apply a prepend without moving the user's logical viewport.

use crate::tui::app::state::AppState;

impl AppState {
    pub(crate) fn apply_pending_history_anchor(&mut self, total_lines: usize) {
        let Some((inserted, old_scroll, rewind)) = self.history_page.take_anchor(total_lines)
        else {
            return;
        };
        let anchored = inserted.saturating_add(old_scroll);
        self.chat_scroll = anchored.saturating_sub(rewind);
        self.chat_auto_follow = false;
        let remaining = rewind.saturating_sub(anchored);
        if remaining > 0 {
            self.history_page.request_older(remaining);
        }
    }
}
