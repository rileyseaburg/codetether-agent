//! Report the original success-marker interpretation of resolver output.
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) fn report_conflict_resolution(
        story: &UserStory,
        output: &str,
    ) -> anyhow::Result<bool> {
        // Check if resolution was successful
        let resolved = codetether_ralph_core::outcome::conflict_resolved(output);

        if resolved {
            info!(target: "codetether_agent::ralph::ralph_loop::conflict_result", story_id = %story.id, "Conflicts resolved successfully");
        } else {
            warn!(target: "codetether_agent::ralph::ralph_loop::conflict_result",
                story_id = %story.id,
                output = %output.chars().take(200).collect::<String>(),
                "Conflict resolution may have failed"
            );
        }

        Ok(resolved)
    }
}
