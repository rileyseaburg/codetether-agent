//! Session identity and restart scheduling around an editor interaction.
use super::Editor;
use crate::tui::app::session_runtime::{SessionSlot, TuiSessionHandle};
pub(crate) struct Runtime<'a> {
    pub editor: Editor<'a>,
    pub slot: &'a SessionSlot,
    pub runtime: &'a TuiSessionHandle,
}
impl<'a> codetether_goal_editor::Session for Runtime<'a> {
    type Editor = Editor<'a>;
    fn editor(&self) -> &Self::Editor {
        &self.editor
    }
    fn editor_mut(&mut self) -> &mut Self::Editor {
        &mut self.editor
    }
    fn session_id(&self) -> &str {
        self.slot.view().id()
    }
    fn restart(&mut self, id: &str) {
        crate::tui::app::input::goal_answer::commands::restart(self.editor.app, id, self.runtime);
    }
}
