use super::Native;
use codetether_chat_submit::queued::{Input, State};
impl Input for Native<'_> {
    fn input(&mut self) -> State<'_> {
        State {
            input: &mut self.app.state.input,
            cursor: &mut self.app.state.input_cursor,
            status: &mut self.app.state.status,
        }
    }
    async fn enter(&mut self) {
        crate::tui::app::input::handle_enter(
            self.app,
            self.cwd,
            self.slot,
            self.registry,
            self.worker_bridge,
            self.runtime,
        )
        .await;
    }
}
