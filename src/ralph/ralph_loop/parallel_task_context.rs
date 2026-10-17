//! Borrow execution inputs without shortening task-owned lifetimes.
use super::parallel_execution_context::ParallelExecutionContext;
use super::parallel_task::ParallelTask;
use super::*;
impl ParallelTask {
    pub(super) fn execution_context<'a>(
        &'a self,
        story_working_dir: &'a PathBuf,
        bridge_tx: &'a Option<mpsc::Sender<SwarmEvent>>,
    ) -> ParallelExecutionContext<'a> {
        parallel_execution_context::ParallelExecutionContext {
            provider: &self.provider,
            model: &self.model,
            story: &self.story,
            working_dir: story_working_dir,
            ralph_tx: &self.ralph_tx,
            bridge_tx: bridge_tx,
            bus: &self.bus,
            registry: &self.registry,
            delegation: self.delegation.as_ref(),
            relay_enabled: self.relay_enabled,
            relay_max_agents: self.relay_max_agents,
            relay_max_rounds: self.relay_max_rounds,
            max_steps_per_story: self.max_steps_per_story,
        }
    }
}
