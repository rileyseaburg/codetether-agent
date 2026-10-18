//! Verify the main workspace after a resolved merge is completed.
use super::*;
pub(crate) async fn verify_resolved_merge<H: Host>(
    host: &mut H,
    working_dir: &Path,
    story: &UserStory,
) {
    info!(target: "codetether_agent::ralph::ralph_loop::conflict_verify",
        story_id = %story.id,
        "Merge completed after conflict resolution"
    );
    match H::verify(working_dir, story).await {
        Ok(()) => host.state_mut().prd.mark_passed(&story.id),
        Err(e) => {
            let error = format!("Verification failed after conflict resolution: {e}");
            warn!(target: "codetether_agent::ralph::ralph_loop::conflict_verify",
                story_id = %story.id,
                error = %error
            );
            host.record_story_result(story, false, Some(error));
            host.try_send_event(RalphEvent::StoryComplete {
                story_id: story.id.clone(),
                passed: false,
            });
        }
    }
}
