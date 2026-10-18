//! Session identity and native command actions for shared input routing.
use super::Native;
impl codetether_goal_command::input::Actions for Native<'_> {
    fn session_id(&self) -> String {
        self.slot.view().id().to_string()
    }
    async fn goal(&mut self, id: &str, text: &str) -> bool {
        crate::tui::app::commands::goal::handle(self.app, id, text).await
    }
    fn tasks(&self, id: &str) -> anyhow::Result<codetether_goal_core::TaskState> {
        crate::session::tasks::runtime::answer_review::read(id)
    }
    fn edit(&mut self, id: &str) {
        super::super::super::editor::open(self.app, id);
    }
    fn restart(&mut self, id: &str) {
        super::super::resume::restart(self.app, id, self.runtime);
    }
}
