//! Route a worktree merge result to success, conflict or failure handlers.
use super::parallel_result_types::MergeContext;
use super::*;
impl RalphLoop {
    pub(super) async fn apply_worktree_merge(
        &mut self,
        ctx: &MergeContext<'_>,
        result: anyhow::Result<crate::worktree::MergeResult>,
    ) {
        match result {
            Ok(merge_result) => {
                if merge_result.success {
                    if !self
                        .complete_successful_merge(
                            ctx.story,
                            ctx.working_dir,
                            ctx.wt,
                            ctx.mgr,
                            &merge_result,
                        )
                        .await
                    {
                        return;
                    }
                } else if !merge_result.conflicts.is_empty() {
                    self.resolve_parallel_merge(
                        ctx.provider,
                        ctx.model,
                        ctx.working_dir,
                        ctx.story,
                        ctx.wt,
                        ctx.mgr,
                        &merge_result,
                    )
                    .await;
                } else {
                    self.report_nonconflict_merge(ctx.story, ctx.wt, ctx.mgr, &merge_result)
                        .await;
                }
            }
            Err(e) => {
                warn!(
                    story_id = %ctx.story.id,
                    error = %e,
                    "Failed to merge worktree"
                );
            }
        }
    }
}
