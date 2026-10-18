//! Bind the shared dependency scheduler to complete stage execution.
use crate::*;
struct Runtime<'a, H: Host> {
    host: &'a mut H,
}
/// Run dependency stages with the host's original iteration and worktree policy.
/// # Arguments
/// * `host` - Native state and successful-story merge service.
/// # Returns
/// Completion after the available stages or configured iteration budget.
/// # Errors
/// Returns PRD save or stage finalization errors.
/// # Examples
/// ```rust,no_run
/// # use codetether_ralph_stage::{run, Host};
/// # async fn example(host: &mut impl Host) -> anyhow::Result<()> {
/// run(host).await
/// # }
/// ```
pub async fn run(host: &mut impl Host) -> anyhow::Result<()> {
    codetether_ralph_core::scheduling::run_parallel(&mut Runtime { host }).await
}
/// Prepare, execute, collect and checkpoint one dependency stage.
/// # Arguments
/// Receives the live host, owned ready stories and the shared optional worktree manager.
/// # Returns
/// Completion after task collection and stage checkpointing.
/// # Errors
/// Returns PRD save errors during finalization.
/// # Examples
/// ```rust,no_run
/// # use codetether_ralph_stage::{Host, execute_stage};
/// # async fn example(host: &mut impl Host) -> anyhow::Result<()> {
/// execute_stage(host, vec![], &None).await
/// # }
/// ```
pub async fn execute_stage<H: Host>(
    host: &mut H,
    stories: Vec<UserStory>,
    worktree_mgr: &Option<Arc<WorktreeManager>>,
) -> anyhow::Result<()> {
    let worktrees_by_story = precreate_stage_worktrees(worktree_mgr.as_ref(), &stories).await;
    let stage = spawn_parallel_stage(host, stories, worktree_mgr, worktrees_by_story);
    collect_parallel_results(
        host,
        stage.handles,
        &stage.provider,
        &stage.model,
        &stage.working_dir,
    )
    .await;
    finish_parallel_stage(host).await?;
    Ok(())
}
#[async_trait::async_trait]
impl<H: Host> codetether_ralph_core::scheduling::ParallelHost for Runtime<'_, H> {
    type Manager = Arc<WorktreeManager>;
    fn stages(&self) -> Vec<Vec<UserStory>> {
        parallel_stages(self.host)
    }
    fn manager(&self) -> Option<Self::Manager> {
        parallel_worktree_manager(self.host)
    }
    fn begin(&mut self, index: usize, total: usize, stories: &[UserStory]) -> bool {
        begin_parallel_stage(self.host, index, total, stories)
    }
    async fn execute_stage(
        &mut self,
        stories: Vec<UserStory>,
        manager: &Option<Self::Manager>,
    ) -> anyhow::Result<()> {
        execute_stage(self.host, stories, manager).await
    }
}
