//! Route a worktree merge result to success, conflict or failure handlers.
use super::*;
pub(crate) async fn apply_worktree_merge<H: Host>(
    host: &mut H,
    ctx: &MergeContext<'_>,
    result: anyhow::Result<MergeResult>,
) {
    match result {
        Ok(merge_result) => {
            if merge_result.success {
                if !complete_successful_merge(
                    host,
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
                resolve_parallel_merge(
                    host,
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
                report_nonconflict_merge(ctx.story, ctx.wt, ctx.mgr, &merge_result).await;
            }
        }
        Err(e) => {
            warn!(target: "codetether_agent::ralph::ralph_loop::merge_dispatch",
                story_id = %ctx.story.id,
                error = %e,
                "Failed to merge worktree"
            );
        }
    }
}
