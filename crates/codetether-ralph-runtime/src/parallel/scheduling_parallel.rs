//! Preserve the public native scheduler interface through shared stage functions.
use super::*;
#[async_trait::async_trait]
impl<H: Host> codetether_ralph_core::scheduling::ParallelHost for RalphLoop<H> {
    type Manager = Arc<WorktreeManager>;
    fn stages(&self) -> Vec<Vec<UserStory>> {
        codetether_ralph_stage::parallel_stages(self)
    }
    fn manager(&self) -> Option<Self::Manager> {
        codetether_ralph_stage::parallel_worktree_manager(self)
    }
    fn begin(&mut self, index: usize, total: usize, stories: &[UserStory]) -> bool {
        codetether_ralph_stage::begin_parallel_stage(self, index, total, stories)
    }
    async fn execute_stage(
        &mut self,
        stories: Vec<UserStory>,
        worktree_mgr: &Option<Self::Manager>,
    ) -> anyhow::Result<()> {
        codetether_ralph_stage::execute_stage(self, stories, worktree_mgr).await
    }
}
