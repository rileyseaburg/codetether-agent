//! Render merge guidance or incomplete-story retry instructions.

use codetether_ralph_core::RalphState;
use std::path::Path;
pub(crate) fn render(
    state: &RalphState,
    prd_path: &Path,
    feature_branch: &str,
    all_passed: bool,
) -> String {
    if all_passed {
        format!(
            "\n## Next Steps\n\n1. Review the changes on branch `{}`\n2. Create a pull request or merge to main:\n   ```bash\n   git checkout main && git merge {} --no-ff\n   ```\n3. Push the changes:\n   ```bash\n   git push\n   ```",
            feature_branch, feature_branch
        )
    } else {
        let failed_stories: Vec<_> = state
            .prd
            .user_stories
            .iter()
            .filter(|s| !s.passes)
            .map(|s| format!("- {}: {}", s.id, s.title))
            .collect();
        format!(
            "\n## Incomplete Stories\n\n{}\n\n## Next Steps\n\n1. Review progress.txt for learnings\n2. Either:\n   - Re-run Ralph: `ralph({{action: 'run', prd_path: '{}'}})`\n   - Fix manually on branch `{}`\n   - Reset PRD to retry: edit {} and set `passes: false`",
            failed_stories.join("\n"),
            prd_path.display(),
            feature_branch,
            prd_path.display()
        )
    }
}
