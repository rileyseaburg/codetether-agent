//! Coordinate conflict prompt construction, execution and result reporting.
use super::*;
impl RalphLoop {
    /// Resolve merge conflicts using a dedicated sub-agent
    pub(in crate::ralph::ralph_loop) async fn resolve_conflicts_static(
        provider: &Arc<dyn Provider>,
        model: &str,
        working_dir: &Path,
        story: &UserStory,
        conflicts: &[String],
        conflict_diffs: &[(String, String)],
        bus: Option<Arc<AgentBus>>,
    ) -> anyhow::Result<bool> {
        info!(target: "codetether_agent::ralph::ralph_loop::conflicts",
            story_id = %story.id,
            num_conflicts = conflicts.len(),
            "Starting conflict resolution sub-agent"
        );

        let prompt = codetether_ralph_core::conflict_prompt::build(
            story,
            conflicts,
            conflict_diffs,
            working_dir,
        );
        let output = Self::execute_conflict_resolution(
            provider,
            model,
            working_dir,
            &story.id,
            &prompt,
            bus,
        )
        .await?;
        Self::report_conflict_resolution(story, &output)
    }
}
