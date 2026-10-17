//! status_markdown formatting.
use crate::RalphState;
/// Render status_markdown output.
///
/// # Arguments
/// * `state` - Current Ralph execution state.
/// # Returns
/// Markdown status report.
/// # Examples
/// ```
/// # fn example(state: &codetether_ralph_core::RalphState) {
/// let text = codetether_ralph_core::presentation::status_markdown(state);
/// assert!(text.contains("## Stories"));
/// # }
/// ```

/// Format status as markdown
pub fn status_markdown(state: &RalphState) -> String {
    let status = if state.prd.is_complete() {
        "# Ralph Complete!"
    } else {
        "# Ralph Status"
    };

    let stories: Vec<String> = state
        .prd
        .user_stories
        .iter()
        .map(|s| {
            let check = if s.passes { "[x]" } else { "[ ]" };
            format!("- {} {}: {}", check, s.id, s.title)
        })
        .collect();

    format!(
        "{}\n\n**Project:** {}\n**Feature:** {}\n**Progress:** {}/{} stories\n**Iterations:** {}/{}\n\n## Stories\n{}",
        status,
        state.prd.project,
        state.prd.feature,
        state.prd.passed_count(),
        state.prd.user_stories.len(),
        state.current_iteration,
        state.max_iterations,
        stories.join("\n")
    )
}
