//! Render the completed Ralph state with cleanup and next steps.

use codetether_ralph_core::RalphState;
use std::path::Path;
#[allow(clippy::too_many_arguments)]
pub(crate) fn run(
    state: &RalphState,
    prd_path: &Path,
    feature_branch: &str,
    all_passed: bool,
    passed_count: usize,
    total_count: usize,
    cleanup_count: usize,
    returned_to_original: bool,
    starting_branch: &Option<String>,
) -> String {
    let next_steps = crate::next_steps::render(state, prd_path, feature_branch, all_passed);
    let (cleanup_note, branch_note) =
        crate::notes::render(cleanup_count, returned_to_original, starting_branch);
    format!(
        "# Ralph {:?}\n\n**Project:** {}\n**Feature:** {}\n**Progress:** {}/{} stories\n**Iterations:** {}/{}\n**Feature Branch:** {}\n\n## Stories\n{}{}{}\n{}",
        state.status,
        state.prd.project,
        state.prd.feature,
        passed_count,
        total_count,
        state.current_iteration,
        state.max_iterations,
        feature_branch,
        state
            .prd
            .user_stories
            .iter()
            .map(|s| format!(
                "- [{}] {}: {}",
                if s.passes { "x" } else { " " },
                s.id,
                s.title
            ))
            .collect::<Vec<_>>()
            .join("\n"),
        cleanup_note,
        branch_note,
        next_steps
    )
}
