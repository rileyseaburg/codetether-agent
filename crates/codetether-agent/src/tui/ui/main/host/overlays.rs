//! Bind global overlays to application state.
use super::Native;
use ratatui::Frame;
impl codetether_tui_render::Overlays for Native<'_> {
    fn symbols_active(&self) -> bool {
        self.app.state.symbol_search.active
    }
    fn symbols(&mut self, frame: &mut Frame) {
        crate::tui::symbol_search::render_symbol_search(
            frame,
            &mut self.app.state.symbol_search,
            frame.area(),
        );
    }
    fn goal_prompt(&mut self, frame: &mut Frame) {
        crate::tui::ui::goal_prompt_overlay::render_if_active(
            frame,
            frame.area(),
            &self.app.state.goal_prompt,
        );
    }
    fn fuzzy_find(&mut self, frame: &mut Frame) {
        crate::tui::ui::fuzzy_find_overlay::render_if_active(
            frame,
            frame.area(),
            &self.app.state.fuzzy_find,
        );
    }
    fn interlude(&mut self, frame: &mut Frame) {
        crate::tui::ui::interlude::render_if_active(frame, frame.area(), self.app);
    }
    fn watchdog(&mut self, frame: &mut Frame) {
        crate::tui::app::watchdog::render_watchdog_notification(
            frame,
            frame.area(),
            &self.app.state,
        );
    }
    fn answer_review(&mut self, frame: &mut Frame) {
        crate::tui::ui::answer_review::render(frame, self.app);
    }
    fn question(&mut self, frame: &mut Frame) {
        crate::tui::ui::question::render(frame, self.app);
    }
}
