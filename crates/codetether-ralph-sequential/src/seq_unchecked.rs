//! Sequential Ralph unchecked handling.
use crate::*;
use codetether_ralph_core::events::RalphEvent;

pub(crate) async fn finish_unchecked_story(
    host: &mut impl Host,
    story: &UserStory,
) -> anyhow::Result<()> {
    // No quality checks; verification still gates completion.
    if let Err(error) = host.verify(&host.state().working_dir, story).await {
        let error = format!("Verification steps failed: {error}");
        warn!(target: "codetether_agent::ralph::ralph_loop::seq_unchecked", story_id = %story.id, error = %error);
        host.record_story_result(story, false, Some(error.clone()));
        host.try_send_event(RalphEvent::StoryComplete {
            story_id: story.id.clone(),
            passed: false,
        });
        return Ok(());
    }
    host.state_mut().prd.mark_passed(&story.id);
    host.state().prd.save(&host.state().prd_path).await?;

    // Record story result in store
    host.record_story_result(story, true, None);

    host.try_send_event(RalphEvent::StoryComplete {
        story_id: story.id.clone(),
        passed: true,
    });
    Ok(())
}
