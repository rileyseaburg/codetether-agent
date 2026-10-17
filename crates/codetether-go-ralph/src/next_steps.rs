//! Render review or retry guidance for a completed Ralph result.

use crate::GoRalphResult;
pub(crate) fn render(result: &GoRalphResult) -> String {
    if result.all_passed {
        format!(
            "\nNext steps:\n  1. Review changes on branch `{}`\n  2. Merge: git checkout main && git merge {} --no-ff\n  3. Push: git push",
            result.feature_branch, result.feature_branch
        )
    } else {
        let failed: Vec<String> = result
            .stories
            .iter()
            .filter(|s| !s.passed)
            .map(|s| format!("  - {}: {}", s.id, s.title))
            .collect();
        format!(
            "\nIncomplete stories:\n{}\n\nNext steps:\n  1. Review progress file for learnings\n  2. Re-run with a clean objective: codetether run -- '/go <concise-task>'\n  3. Or fix manually on branch `{}`",
            failed.join("\n"),
            result.feature_branch
        )
    }
}
