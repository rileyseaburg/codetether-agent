//! Bind selection and native sessions to shared child-detail routing.
use crate::tui::app::state::AppState;
use ratatui::text::Line;
struct Selected<'a>(&'a AppState);
impl codetether_agent_dashboard::detail::Host for Selected<'_> {
    fn selected(&self) -> Option<&str> {
        self.0.active_spawned_agent.as_deref()
    }
    fn managed(&self, name: &str) -> Option<Vec<Line<'static>>> {
        self.0
            .spawned_agents
            .get(name)
            .map(super::subagent_detail_managed::lines)
    }
    fn tool(&self, name: &str) -> Vec<Line<'static>> {
        super::subagent_detail_tool::lines(self.0, name)
    }
}
pub(super) fn lines(state: &AppState) -> Vec<Line<'static>> {
    codetether_agent_dashboard::detail::lines(&Selected(state))
}

#[cfg(test)]
#[path = "subagent_detail_lines_tests.rs"]
mod tests;
