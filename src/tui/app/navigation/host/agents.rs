//! Native agents operations requested by navigation policy.
use crate::tui::app::state::App;
use codetether_tui_navigation::Action;
pub(super) fn apply(app: &mut App, action: &Action) -> bool {
    match action {
        Action::SwarmPrevious => {
            app.state.swarm.select_prev();
        }
        Action::SwarmNext => {
            app.state.swarm.select_next();
        }
        Action::RalphPrevious => {
            app.state.ralph.select_prev();
        }
        Action::RalphNext => {
            app.state.ralph.select_next();
        }
        Action::BusPrevious => {
            app.state.bus_log.select_prev();
        }
        Action::BusNext => {
            app.state.bus_log.select_next();
        }
        Action::ChildPrevious => {
            super::super::agent_focus::cycle_child_focus_back(app);
        }
        Action::ChildNext => {
            super::super::agent_focus::cycle_child_focus(app);
        }
        _ => return false,
    }
    true
}
