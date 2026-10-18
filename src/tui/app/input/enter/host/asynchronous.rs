use super::Native;
impl codetether_tui_navigation::enter::Async for Native<'_> {
    async fn sessions(&mut self) {
        super::super::mode::sessions(
            self.app,
            self.cwd,
            self.slot,
            self.registry,
            self.worker_bridge,
            self.runtime,
        )
        .await;
    }
    async fn swarm(&mut self) {
        super::super::enter_swarm::dispatch_swarm_enter(
            self.app,
            self.cwd,
            self.slot,
            self.registry,
            self.worker_bridge,
            self.runtime,
        )
        .await;
    }
    async fn chat(&mut self) {
        super::super::super::chat_submit::handle_enter_chat(
            self.app,
            self.cwd,
            self.slot,
            self.registry,
            self.worker_bridge,
            self.runtime,
        )
        .await;
    }
    async fn settings(&mut self) {
        super::super::mode::settings(self.app, self.slot).await;
    }
}
