use super::Native;
use crate::tui::app::state::prompt_queue;
impl codetether_chat_submit::queued::Queue for Native<'_> {
    fn processing(&self) -> bool {
        self.app.state.processing
    }
    fn has_pending(&self) -> bool {
        prompt_queue::has_pending()
    }
    fn held(&self) -> bool {
        crate::session::tasks::runtime::answer_review::held(self.slot.view().id())
    }
    fn take(&mut self) -> Option<String> {
        prompt_queue::take()
    }
}
