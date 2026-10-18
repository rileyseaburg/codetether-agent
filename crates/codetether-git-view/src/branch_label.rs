//! Branch, detached-head and unavailable-repository display labels.
use crate::GitViewState;

pub(crate) fn branch_label(state: &GitViewState) -> String {
    if let Some(branch) = state.branch.as_deref() {
        return branch.to_string();
    }

    let has_git_data = !state.branches.is_empty()
        || !state.log_lines.is_empty()
        || !state.diff_stat.trim().is_empty()
        || state.dirty_files > 0;

    if has_git_data {
        "(detached HEAD)".to_string()
    } else {
        "(not a git repo)".to_string()
    }
}
