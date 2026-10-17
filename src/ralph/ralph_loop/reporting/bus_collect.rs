//! Drain and format inter-iteration learnings from the agent bus.
use super::*;
impl RalphLoop {
    /// Collect accumulated learnings from the bus for injection into prompts.
    pub(in crate::ralph::ralph_loop) fn bus_collect_learnings(&self) -> Vec<String> {
        let Some(ref bus) = self.bus else {
            return Vec::new();
        };
        codetether_ralph_core::bus::collect(bus, &self.state.prd.project)
    }
}
