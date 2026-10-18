//! Native input/history binding for shared raw prompt capture.
use crate::tui::app::state::App;
impl codetether_chat_submit::history::History for App {
    fn input(&self) -> &str {
        &self.state.input
    }
    fn push_history(&mut self, prompt: String) {
        self.state.push_history(prompt);
    }
}
pub(super) fn capture(app: &mut App) -> String {
    codetether_chat_submit::history::capture(app)
}
