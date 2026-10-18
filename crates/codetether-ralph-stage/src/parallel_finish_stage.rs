//! Increment once, publish stage learnings, and persist the updated PRD.
use super::*;
pub(crate) async fn finish_parallel_stage<H: Host>(host: &mut H) -> anyhow::Result<()> {
    // Increment iteration once per stage, not per story
    host.state_mut().current_iteration += 1;

    // Publish stage-level progress to bus after all stories in this stage
    if host.inputs().bus.is_some() {
        let bus_learnings: Vec<String> = host
            .inputs()
            .state
            .progress_log
            .iter()
            .flat_map(|e| e.learnings.clone())
            .collect();
        for story in &host.inputs().state.prd.user_stories {
            if story.passes {
                bus_publish_story_result(
                    host,
                    story,
                    host.inputs().state.current_iteration,
                    &bus_learnings,
                    None,
                );
            }
        }
    }

    // Save PRD after each stage
    host.inputs()
        .state
        .prd
        .save(&host.inputs().state.prd_path)
        .await?;

    // Persist PRD + iteration to store after each stage
    codetether_ralph_core::state_store::updates::checkpoint(
        host.inputs().store,
        host.inputs().run_id,
        host.inputs().state,
    );
    Ok(())
}
