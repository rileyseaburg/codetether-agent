//! Append progress and route each joined task to merge or failure reporting.
use super::parallel_result_types::ParallelStoryResult;
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) async fn apply_parallel_result(
        &mut self,
        result: ParallelStoryResult,
        provider: &Arc<dyn Provider>,
        model: &str,
        working_dir: &Path,
    ) {
        let (story, success, entry, worktree_info, worktree_mgr, quality_passed, failure_reason) =
            result;
        self.state.progress_log.push(entry);

        if success && quality_passed {
            // Quality gates already ran (with retry/repair) inside the spawned task.
            // Proceed directly to merge.
            {
                self.merge_parallel_story(
                    provider,
                    model,
                    working_dir,
                    &story,
                    worktree_info.as_ref(),
                    worktree_mgr.as_ref(),
                )
                .await;
            }
        } else {
            self.report_failed_parallel_story(
                &story,
                success,
                failure_reason,
                worktree_info.as_ref(),
            );
        }
    }
}
