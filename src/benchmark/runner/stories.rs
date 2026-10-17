//! Map Ralph progress entries into per-story benchmark results.
use crate::ralph::RalphState;
use codetether_benchmark::StoryBenchmarkResult;
pub(super) fn story_results(state: &RalphState) -> Vec<StoryBenchmarkResult> {
    // Build per-story results from Ralph's state
    let per_story: Vec<StoryBenchmarkResult> = state
        .prd
        .user_stories
        .iter()
        .map(|story| {
            let progress_entries: Vec<_> = state
                .progress_log
                .iter()
                .filter(|p| p.story_id == story.id)
                .collect();

            StoryBenchmarkResult {
                story_id: story.id.clone(),
                title: story.title.clone(),
                passed: story.passes,
                iterations: progress_entries.len(),
                duration_seconds: 0.0, // Per-story timing not tracked in Ralph
                tokens_used: 0,        // Per-story tokens not tracked separately
                files_changed: progress_entries
                    .iter()
                    .flat_map(|p| p.files_changed.iter().cloned())
                    .collect::<std::collections::HashSet<_>>()
                    .into_iter()
                    .collect(),
            }
        })
        .collect();

    per_story
}
