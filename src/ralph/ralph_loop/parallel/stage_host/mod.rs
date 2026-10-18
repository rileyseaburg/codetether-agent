//! Native state view and merge binding for the shared stage coordinator.
//!
//! Task execution uses the existing relay/model host; stage orchestration and
//! checkpoint ordering live in codetether-ralph-stage.
use super::*;
mod merge;
#[async_trait::async_trait]
impl codetether_ralph_stage::Host for RalphLoop {
    type Task = super::host::Native;
    fn inputs(&self) -> codetether_ralph_stage::Inputs<'_, Self::Task> {
        codetether_ralph_stage::Inputs {
            state: &self.state,
            config: &self.config,
            provider: &self.provider,
            model: &self.model,
            event_tx: &self.event_tx,
            bus: &self.bus,
            registry: &self.registry,
            delegation: &self.delegation,
            store: self.store.as_ref(),
            run_id: &self.run_id,
        }
    }
    fn state_mut(&mut self) -> &mut RalphState {
        &mut self.state
    }
    async fn merge_parallel_story(
        &mut self,
        provider: &Arc<dyn Provider>,
        model: &str,
        working_dir: &Path,
        story: &UserStory,
        worktree: Option<&crate::worktree::WorktreeInfo>,
        manager: Option<&Arc<WorktreeManager>>,
    ) {
        merge::run(self, provider, model, working_dir, story, worktree, manager).await;
    }
}
