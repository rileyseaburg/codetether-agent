//! Clone the stage inputs in the original capture order for one task.
use super::parallel_task::ParallelTask;
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) fn prepare_parallel_task(
        &self,
        semaphore: &Arc<tokio::sync::Semaphore>,
        provider: &Arc<dyn Provider>,
        model: &String,
        prd_info: &(String, String),
        working_dir: &PathBuf,
        worktree_mgr: &Option<Arc<WorktreeManager>>,
        worktrees_by_story: &mut HashMap<String, crate::worktree::WorktreeInfo>,
        progress_path: &String,
        accumulated_learnings: &Vec<String>,
        story: UserStory,
    ) -> ParallelTask {
        ParallelTask {
            sem: Arc::clone(semaphore),
            provider: Arc::clone(provider),
            model: model.clone(),
            prd_info: prd_info.clone(),
            working_dir: working_dir.clone(),
            worktree_mgr: worktree_mgr.clone(),
            prebuilt_worktree: worktrees_by_story.remove(&story.id),
            progress_path: progress_path.clone(),
            ralph_tx: self.event_tx.clone(),
            stage_learnings: accumulated_learnings.clone(),
            bus: self.bus.clone(),
            relay_enabled: self.config.relay_enabled,
            relay_max_agents: self.config.relay_max_agents,
            relay_max_rounds: self.config.relay_max_rounds,
            registry: self.registry.clone(),
            max_steps_per_story: self.config.max_steps_per_story,
            max_quality_retries: self.config.max_quality_retries,
            max_rate_limit_retries: self.config.max_rate_limit_retries,
            rate_limit_base_delay_ms: self.config.rate_limit_base_delay_ms,
            quality_checks_enabled: self.config.quality_checks_enabled,
            quality_checks: self.state.prd.quality_checks.clone(),
            delegation: self.delegation.clone(),
            story: story,
        }
    }
}
