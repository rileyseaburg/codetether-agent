//! Native swarm selection bindings for shared Tab routing.
use super::Navigation;

impl codetether_tui_navigation::tab::Host for Navigation<'_> {
    fn select_previous_swarm_task(&mut self) {
        self.0.state.swarm.select_prev();
    }

    fn select_next_swarm_task(&mut self) {
        self.0.state.swarm.select_next();
    }
}
