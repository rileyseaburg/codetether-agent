//! Sequential Ralph success handling.
use crate::*;
use codetether_ralph_core::events::RalphEvent;

pub(crate) async fn finish_verified_story(
    host: &mut impl Host,
    story: &UserStory,
    entry_learnings: &[String],
) -> anyhow::Result<()> {
    info!(target: "codetether_agent::ralph::ralph_loop::seq_success", "Story {} passed quality checks!", story.id);
    if let Err(error) = host.verify(&host.state().working_dir, story).await {
        let error = format!("Verification steps failed: {error}");
        warn!(target: "codetether_agent::ralph::ralph_loop::seq_success", story_id = %story.id, error = %error);
        host.record_story_result(story, false, Some(error.clone()));
        host.try_send_event(RalphEvent::StoryComplete {
            story_id: story.id.clone(),
            passed: false,
        });
        return Ok(());
    }
    host.state_mut().prd.mark_passed(&story.id);

    // Record story result in store
    host.record_story_result(story, true, None);

    host.try_send_event(RalphEvent::StoryComplete {
        story_id: story.id.clone(),
        passed: true,
    });

    // Publish learnings to bus
    let next = host.state().prd.next_story().map(|s| s.id.clone());
    host.bus_publish_story_result(
        story,
        host.state().current_iteration,
        entry_learnings,
        next.as_deref(),
    );

    // Commit changes
    if host.config().auto_commit {
        host.commit_story(story)?;
    }

    // Save updated PRD
    host.state().prd.save(&host.state().prd_path).await?;
    Ok(())
}
