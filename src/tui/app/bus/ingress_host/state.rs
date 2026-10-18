//! Native cursor and identity state for bus ingress.
use super::Native;
use std::collections::HashSet;
impl codetether_bus_view::ingress::State for Native<'_> {
    fn cursor(&mut self) -> &mut u64 {
        &mut self.0.state.bus_cursor
    }
    fn agents(&mut self) -> &mut HashSet<String> {
        &mut self.0.state.worker.worker_bridge_registered_agents
    }
    fn is_registered(&self, agent: &str) -> bool {
        self.0
            .state
            .worker
            .worker_bridge_registered_agents
            .contains(agent)
    }
    fn worker_id(&self) -> Option<&str> {
        self.0.state.worker.worker_id.as_deref()
    }
    fn session_id(&self) -> Option<&str> {
        self.0.state.session_id.as_deref()
    }
}
