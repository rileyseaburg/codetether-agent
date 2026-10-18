use super::Native;
use crate::tui::app::input::{chat_helpers::no_provider_error, chat_spawn::spawn_provider_task};
use crate::{provider::ProviderRegistry, session::ImageAttachment};
use codetether_chat_submit::dispatch::{Host, State};
use std::sync::Arc;
impl Host for Native<'_> {
    type Provider = Arc<ProviderRegistry>;
    type Image = ImageAttachment;
    fn held(&self) -> bool {
        crate::session::tasks::runtime::answer_review::held(self.slot.view().id())
    }
    fn clear_input(&mut self) {
        self.app.state.clear_input();
    }
    async fn processing_started(&mut self) {
        crate::tui::app::worker_bridge::handle_processing_started(self.app, self.worker_bridge)
            .await;
    }
    fn begin_timing(&mut self) {
        self.app.state.begin_request_timing();
    }
    fn state(&mut self) -> State<'_> {
        State {
            watchdog_restarts: &mut self.app.state.main_watchdog_restart_count,
            inflight_prompt: &mut self.app.state.main_inflight_prompt,
            status: &mut self.app.state.status,
        }
    }
    fn scroll(&mut self) {
        self.app.state.scroll_to_bottom();
    }
    async fn spawn(&mut self, provider: &Self::Provider, prompt: &str, images: Vec<Self::Image>) {
        spawn_provider_task(
            self.app,
            self.cwd,
            self.slot,
            provider,
            prompt,
            images,
            self.runtime,
        )
        .await;
    }
    async fn no_provider(&mut self) {
        no_provider_error(self.app, self.worker_bridge).await;
    }
}
