//! Verify and report a completed story that used the main working directory.
use super::*;
pub(crate) async fn complete_story_without_worktree<H: Host>(
    host: &mut H,
    story: &UserStory,
    working_dir: &Path,
) -> bool {
    // No worktree; verification still gates completion.
    if let Err(error) = H::verify(working_dir, story).await {
        let error = format!("Verification steps failed: {error}");
        warn!(target: "codetether_agent::ralph::ralph_loop::merge_without_worktree", story_id = %story.id, error = %error);
        host.record_story_result(story, false, Some(error));
        host.try_send_event(RalphEvent::StoryComplete {
            story_id: story.id.clone(),
            passed: false,
        });
        return false;
    }
    host.state_mut().prd.mark_passed(&story.id);
    host.try_send_event(RalphEvent::StoryComplete {
        story_id: story.id.clone(),
        passed: true,
    });

    // Record story result in store
    host.record_story_result(story, true, None);
    true
}
