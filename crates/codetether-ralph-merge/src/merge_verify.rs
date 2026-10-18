//! Gate successful merges on story verification in the main workspace.
use super::*;
pub(crate) async fn verify_merged_story<H: Host>(
    host: &H,
    story: &UserStory,
    working_dir: &Path,
) -> bool {
    if let Err(error) = H::verify(working_dir, story).await {
        let error = format!("Verification steps failed after merge: {error}");
        warn!(target: "codetether_agent::ralph::ralph_loop::merge_verify", story_id = %story.id, error = %error);
        host.record_story_result(story, false, Some(error));
        host.try_send_event(RalphEvent::StoryComplete {
            story_id: story.id.clone(),
            passed: false,
        });
        return false;
    }
    true
}
