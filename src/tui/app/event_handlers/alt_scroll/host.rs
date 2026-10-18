use crate::tui::{app::state::App, models::ViewMode};
use codetether_tui_navigation::modified::scroll::{Action, Host};
pub(super) struct Native<'a>(pub &'a mut App);
impl Host for Native<'_> {
    fn in_chat(&self) -> bool {
        self.0.state.view_mode == ViewMode::Chat
    }
    fn apply(&mut self, action: Action) {
        match action {
            Action::Down(lines) => self.0.state.chat_viewport.scroll_down(lines),
            Action::Up(lines) => {
                crate::tui::app::state::scroll::view(&mut self.0.state).scroll_up(lines)
            }
            Action::PageDown => crate::tui::app::state::scroll::view(&mut self.0.state).page_down(),
            Action::PageUp => crate::tui::app::state::scroll::view(&mut self.0.state).page_up(),
            Action::Top => self.0.state.chat_viewport.scroll_to_top(),
            Action::Bottom => self.0.state.chat_viewport.scroll_to_bottom(),
        }
    }
}
