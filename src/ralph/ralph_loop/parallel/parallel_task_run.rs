//! Keep the semaphore permit and event bridge alive for the full task.
use super::parallel_result_types::ParallelStoryResult;
use super::parallel_task::ParallelTask;
use super::*;
impl ParallelTask {
    pub(in crate::ralph::ralph_loop) async fn run(mut self) -> ParallelStoryResult {
        let _permit = self.sem.acquire().await.expect("semaphore closed");
        let (story_working_dir, worktree_info) = if let Some(wt) = self.prebuilt_worktree.take() {
            (wt.path.clone(), Some(wt))
        } else {
            (self.working_dir.clone(), None)
        };
        self.announce(&story_working_dir).await;
        let prompt = codetether_ralph_core::presentation::build_parallel_prompt(
            &self.story,
            &self.prd_info,
            &story_working_dir,
            &self.stage_learnings,
        );
        let (bridge_tx, _bridge_handle) = self.event_bridge();
        let execution = self.execution_context(&story_working_dir, &bridge_tx);
        let (final_result, quality_passed, failure_reason) =
            self.execute(&execution, &prompt).await;
        let entry = RalphLoop::parallel_progress_entry(
            &self.story,
            &final_result,
            &story_working_dir,
            &self.progress_path,
        );
        (
            self.story,
            final_result.is_ok(),
            entry,
            worktree_info,
            self.worktree_mgr,
            quality_passed,
            failure_reason,
        )
    }
}
