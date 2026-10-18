//! Read native panel selection and delegate row composition.
use crate::tui::app::state::AppState;
use ratatui::text::Line;
pub(super) struct Panel<'a>(pub &'a AppState);
impl codetether_agent_dashboard::view::Host for Panel<'_> {
    fn detail_mode(&self) -> bool {
        self.0.subagent_detail_mode
    }
    fn selected(&self) -> Option<&str> {
        self.0.active_spawned_agent.as_deref()
    }
    fn scroll(&self) -> u16 {
        self.0.subagent_detail_scroll
    }
    fn dashboard_lines(&self) -> Vec<Line<'static>> {
        super::super::subagent_lines::lines(self.0)
    }
    fn detail_lines(&self) -> Vec<Line<'static>> {
        super::super::subagent_detail_lines::lines(self.0)
    }
}
