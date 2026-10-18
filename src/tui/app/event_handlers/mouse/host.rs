//! Approval and click operations for the shared mouse navigation policy.
use crate::tui::app::navigation::Navigation;
use crossterm::event::KeyCode;
use std::path::Path;
impl codetether_tui_navigation::mouse::Host for Navigation<'_> {
    fn approval_wheel(&mut self, key: KeyCode, amount: usize) -> bool {
        super::super::approval_key::wheel(self.0, key, amount)
    }
    async fn editor_click(&mut self, cwd: &Path, column: u16, row: u16) -> bool {
        super::super::editor_click::editor_click(self.0, cwd, column, row).await
    }
    fn click_open_path(&mut self, cwd: &Path, column: u16, row: u16) {
        super::super::click_open::click_open_path(self.0, cwd, column, row);
    }
    fn audit_previous(&mut self) {
        self.0.state.audit.select_prev();
    }
    fn audit_next(&mut self) {
        self.0.state.audit.select_next();
    }
}
