use super::Native;
use codetether_tui_navigation::events::Layers;
use crossterm::event::KeyEvent;
impl Layers for Native<'_> {
    fn question(&mut self, key: KeyEvent) -> bool {
        super::super::super::paste::question_key::handle(self.app, key)
    }
    async fn editor(&mut self, key: KeyEvent) -> bool {
        crate::tui::app::input::goal_answer::editor::handle(self.app, self.slot, self.runtime, key)
            .await
    }
    fn approval_scroll(&mut self, key: KeyEvent) -> bool {
        super::super::super::approval_key::scroll(self.app, key)
    }
    async fn okr(&mut self, key: KeyEvent) -> bool {
        super::super::super::okr::key::handle(self.app, key).await
    }
}
