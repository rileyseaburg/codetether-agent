use super::Native;
use codetether_tui_navigation::modified::Routing;
use crossterm::event::KeyEvent;
impl Routing for Native<'_> {
    type Error = anyhow::Error;
    fn alt_scroll(&mut self, key: KeyEvent) -> Option<anyhow::Result<bool>> {
        super::super::super::alt_scroll::handle_alt_scroll(self.app, key)
    }
    fn approval(&mut self, character: char) -> bool {
        super::super::super::approval_key::handle(self.app, character, self.cwd)
    }
    fn ctrl_c(&mut self) -> bool {
        super::super::super::ctrl_c::handle_ctrl_c(self.app, self.runtime)
    }
    fn fallback(&mut self, key: KeyEvent, ctrl: bool) -> Option<anyhow::Result<bool>> {
        super::super::super::shared_file_open::ctrl_key(self.app, self.cwd, key, ctrl)
    }
}
