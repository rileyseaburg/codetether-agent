use super::Native;
use crate::tui::app::input;
impl codetether_chat_submit::Intercept for Native<'_> {
    async fn goal_command(&mut self) -> bool {
        input::goal_answer::commands::intercept(self.app, self.slot, self.runtime).await
    }
    async fn goal_answer(&mut self) -> bool {
        input::goal_answer::intercept(self.app, self.slot, self.runtime).await
    }
    fn approval_feedback(&mut self) -> bool {
        input::approval_feedback::submit(self.app)
    }
}
