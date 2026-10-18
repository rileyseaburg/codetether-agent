//! Bind the forage crate to application execution and chat presentation.
use crate::tui::app::state::App;
use crate::tui::chat::message::{ChatMessage, MessageType};
use codetether_forage_interactive::{ForageState, Host, View};
pub(super) struct Runtime;
impl Host for Runtime {
    fn set_quiet(quiet: bool) {
        crate::forage::console::set_quiet(quiet);
    }
    async fn execute(
        args: crate::cli::ForageArgs,
    ) -> anyhow::Result<codetether_forage_core::ForageRunSummary> {
        crate::forage::execute_with_summary(args).await
    }
}
impl View for App {
    fn state(&mut self) -> &mut ForageState {
        &mut self.state.forage
    }
    fn set_status(&mut self, status: String) {
        self.state.status = status;
    }
    fn push_system(&mut self, text: String) {
        self.state
            .messages
            .push(ChatMessage::new(MessageType::System, text));
    }
    fn scroll_to_bottom(&mut self) {
        self.state.scroll_to_bottom();
    }
}
