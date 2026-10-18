use super::{App, Path, SessionSlot, TuiSessionHandle};
use codetether_tui_navigation::events::priority::Host;
use crossterm::event::KeyEvent;
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub cwd: &'a Path,
    pub slot: &'a mut SessionSlot,
    pub runtime: &'a TuiSessionHandle,
}
impl Host for Native<'_> {
    fn interrupt(&mut self, key: KeyEvent) -> Option<bool> {
        super::super::interrupt_key::handle(self.app, self.runtime, key)
    }
    fn interlude(&mut self, key: KeyEvent) -> bool {
        super::super::interlude_key::handle(self.app, key)
    }
    fn goal(&mut self, key: KeyEvent) -> bool {
        super::super::goal_prompt_key::handle_goal_prompt_key(self.app, key)
    }
    fn spawn_active(&self) -> bool {
        self.app.state.spawn_form.is_some()
    }
    async fn spawn(&mut self, key: KeyEvent) -> bool {
        crate::tui::app::spawn_form::handle_spawn_form_key(self.app, self.cwd, self.slot, key).await
    }
    fn fuzzy(&mut self, key: KeyEvent) -> bool {
        super::super::fuzzy_find_key::handle_fuzzy_find_key(self.app, self.cwd, key)
    }
    async fn editor_lsp(&mut self, key: KeyEvent) -> bool {
        super::super::editor_lsp_key::handle_editor_lsp_key(self.app, self.cwd, key).await
    }
    fn editor(&mut self, key: KeyEvent) -> bool {
        super::super::editor_key::handle_editor_key(self.app, self.cwd, key)
    }
}
