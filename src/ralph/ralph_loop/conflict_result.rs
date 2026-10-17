//! Report the original success-marker interpretation of resolver output.
use super::*;
impl RalphLoop {
    pub(super) fn report_conflict_resolution(
        story: &UserStory,
        output: &str,
    ) -> anyhow::Result<bool> {
        // Check if resolution was successful
        let resolved = output.contains("CONFLICTS_RESOLVED")
            || (output.contains("resolved") && !output.contains("UNRESOLVED"));

        if resolved {
            info!(story_id = %story.id, "Conflicts resolved successfully");
        } else {
            warn!(
                story_id = %story.id,
                output = %output.chars().take(200).collect::<String>(),
                "Conflict resolution may have failed"
            );
        }

        Ok(resolved)
    }
}
