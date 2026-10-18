//! Native history binding for shared message-panel scroll clamping.
use crate::tui::app::state::App;
pub use codetether_chat_layout::{SCROLL_BOTTOM, messages::reset_height_memo};
use ratatui::{layout::Rect, text::Line};
impl codetether_chat_layout::messages::Scroll for App {
    fn set_viewport_height(&mut self, height: usize) {
        self.state.history_page.set_viewport_height(height);
    }
    fn apply_pending_anchor(&mut self, total_lines: usize) {
        crate::tui::app::state::history_page::apply_pending_anchor(&mut self.state, total_lines);
    }
    fn viewport(&mut self) -> &mut codetether_chat_layout::ChatScrollState {
        &mut self.state.chat_viewport
    }
}
/// Clamp the message panel after native history anchoring.
pub fn clamp_scroll(app: &mut App, messages_rect: Rect, lines: &[Line<'_>]) -> u16 {
    codetether_chat_layout::messages::clamp(app, messages_rect, lines)
}
