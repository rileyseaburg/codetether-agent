//! Invoke the conflict resolver for the current merge result.
use super::*;
pub(crate) async fn invoke_merge_resolver<H: Host>(
    host: &H,
    provider: &Arc<dyn Provider>,
    model: &str,
    working_dir: &Path,
    story: &UserStory,
    merge_result: &MergeResult,
) -> anyhow::Result<bool> {
    // Real conflicts - spawn conflict resolver
    info!(target: "codetether_agent::ralph::ralph_loop::conflict_invoke",
        story_id = %story.id,
        num_conflicts = merge_result.conflicts.len(),
        "Spawning conflict resolver sub-agent"
    );

    // Try to resolve conflicts
    let resolved = resolve_conflicts_static::<H>(
        provider,
        model,
        working_dir,
        story,
        &merge_result.conflicts,
        &merge_result.conflict_diffs,
        host.bus(),
    )
    .await;
    resolved
}
