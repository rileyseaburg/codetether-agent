//! Execute a parallel story using one provider and its original tool loop.
use super::parallel_execution_context::ParallelExecutionContext;
use super::*;
impl<H: Host> ParallelExecutionContext<'_, H> {
    pub(crate) async fn call_direct(&self, prompt: &str) -> anyhow::Result<String> {
        codetether_ralph_agent::parallel::<H::Agent>(
            self.provider,
            self.model,
            prompt,
            self.working_dir,
            self.bridge_tx.clone(),
            self.story.id.clone(),
            self.bus.clone(),
            self.max_steps_per_story,
        )
        .await
    }
}
