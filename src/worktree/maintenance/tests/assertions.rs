use crate::worktree::maintenance::{WorktreeCleanupReport, WorktreeCleanupState};

pub(super) fn state(report: &WorktreeCleanupReport, name: &str) -> WorktreeCleanupState {
    report
        .entries
        .iter()
        .find(|entry| entry.path.ends_with(name))
        .unwrap()
        .state
}
