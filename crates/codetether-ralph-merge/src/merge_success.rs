//! Mark and report a verified successful worktree merge, then clean up.
use super::*;
pub(crate) async fn complete_successful_merge<H: Host>(
    host: &mut H,
    story: &UserStory,
    working_dir: &Path,
    wt: &WorktreeInfo,
    mgr: &Arc<WorktreeManager>,
    merge_result: &MergeResult,
) -> bool {
    info!(target: "codetether_agent::ralph::ralph_loop::merge_success",
        story_id = %story.id,
        files_changed = merge_result.files_changed,
        "Merged story changes successfully"
    );
    if !verify_merged_story(host, story, working_dir).await {
        let _ = mgr.cleanup(&wt.name).await;
        return false;
    }
    host.state_mut().prd.mark_passed(&story.id);
    host.try_send_event(RalphEvent::StoryMerge {
        story_id: story.id.clone(),
        success: true,
        summary: merge_result.summary.clone(),
    });
    host.try_send_event(RalphEvent::StoryComplete {
        story_id: story.id.clone(),
        passed: true,
    });

    // Record story result in store
    host.record_story_result(story, true, None);

    // Cleanup worktree
    let _ = mgr.cleanup(&wt.name).await;
    true
}
