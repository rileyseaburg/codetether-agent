//! Expose the existing live state and persistence handle.
use super::*;
impl<H: Host> codetether_ralph_sequential::State for RalphLoop<H> {
    fn state(&self) -> &RalphState {
        &self.state
    }
    fn state_mut(&mut self) -> &mut RalphState {
        &mut self.state
    }
    fn config(&self) -> &RalphConfig {
        &self.config
    }
    fn store(&self) -> Option<&Arc<dyn RalphStateStore>> {
        self.store.as_ref()
    }
    fn run_id(&self) -> &str {
        &self.run_id
    }
}
