//! Invoke the conflict resolver for the current merge result.
use super::*;
impl RalphLoop {
    pub(super) async fn invoke_merge_resolver(
        &self,
        provider: &Arc<dyn Provider>,
        model: &str,
        working_dir: &Path,
        story: &UserStory,
        merge_result: &crate::worktree::MergeResult,
    ) -> anyhow::Result<bool> {
        // Real conflicts - spawn conflict resolver
        info!(
            story_id = %story.id,
            num_conflicts = merge_result.conflicts.len(),
            "Spawning conflict resolver sub-agent"
        );

        // Try to resolve conflicts
        let resolved = Self::resolve_conflicts_static(
            provider,
            model,
            working_dir,
            story,
            &merge_result.conflicts,
            &merge_result.conflict_diffs,
            self.bus.clone(),
        )
        .await;
        resolved
    }
}
