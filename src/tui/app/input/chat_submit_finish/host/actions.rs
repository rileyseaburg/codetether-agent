use super::Native;
use crate::tui::app::input::{
    chat_helpers::push_user_messages, chat_submit_dispatch::dispatch_prompt,
};
impl codetether_chat_submit::finish::Actions for Native<'_> {
    async fn adopt(&mut self, agent_prompt: &str) {
        super::super::goal::adopt(self.app, self.slot, agent_prompt).await;
    }
    fn publish(&mut self, prompt: &str, images: &[Self::Image]) {
        push_user_messages(self.app, prompt, images);
    }
    async fn dispatch(&mut self, agent_prompt: &str, images: Vec<Self::Image>) {
        dispatch_prompt(
            self.app,
            self.cwd,
            self.slot,
            self.registry,
            self.worker_bridge,
            agent_prompt,
            images,
            self.runtime,
        )
        .await;
    }
}
