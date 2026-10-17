//! Render a completed autonomous pipeline for CLI and tool callers.

use crate::GoRalphResult;
/// Format story counts, branch, PRD path and review or retry guidance.
///
/// # Arguments
/// * `result` — Terminal pipeline summary.
/// * `task` — Original task description.
///
/// # Returns
/// The original multiline human-readable pipeline result.
///
/// # Examples
/// See [GoRalphResult] for constructing a terminal summary.

/// Format a GoRalphResult for display.
pub fn format_go_ralph_result(result: &GoRalphResult, task: &str) -> String {
    let status_icon = if result.all_passed { "✅" } else { "❌" };
    let status_label = format!("{:?}", result.status);

    let story_lines: Vec<String> = result
        .stories
        .iter()
        .map(|s| {
            format!(
                "  {} {}: {}",
                if s.passed { "✓" } else { "✗" },
                s.id,
                s.title
            )
        })
        .collect();

    let next_steps = crate::next_steps::render(result);
    format!(
        "{status_icon} /go Ralph {status_label}\n\n\
         Task: {task}\n\
         Progress: {passed}/{total} stories | Iterations: {iters}/{max}\n\
         Feature branch: {branch}\n\
         PRD: {prd}\n\n\
         Stories:\n{stories}\n{next}",
        task = task,
        passed = result.passed,
        total = result.total,
        iters = result.iterations,
        max = result.max_iterations,
        branch = result.feature_branch,
        prd = result.prd_path.display(),
        stories = story_lines.join("\n"),
        next = next_steps,
    )
}
