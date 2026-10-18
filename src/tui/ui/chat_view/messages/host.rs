//! Native title, accent and interaction fields for shared panel rendering.
use crate::tui::app::{session_runtime::SessionView, state::App};
use ratatui::{style::Color, text::Line};
pub(super) struct Native<'a> {
    pub app: &'a mut App,
    pub session: &'a SessionView,
}
impl codetether_chat_layout::messages::Host for Native<'_> {
    type Scroll = App;
    fn scroll(&mut self) -> &mut App {
        self.app
    }
    fn accent(&self) -> Color {
        crate::tui::ui::mode_accent::mode_accent(&self.app.state.view_mode)
    }
    fn title(&self) -> Line<'static> {
        super::super::title::build_title(self.app, self.session)
    }
    fn hit(&mut self) -> &mut codetether_chat_layout::ChatHit {
        &mut self.app.state.chat_hit
    }
}
