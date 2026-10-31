//! Shared access does not depend on task wording or worktree hints.
use super::TaskAccess;
use crate::SubTask;

#[test]
fn all_shared_tasks_can_edit_while_isolated_tasks_retain_their_policy() {
    for instruction in ["Inspect code", "Run focused tests", "Edit code"] {
        for preference in [false, true] {
            let task = SubTask::new("worker", instruction).with_needs_worktree(preference);
            let shared = TaskAccess::for_task(&task, true);
            assert!(!shared.read_only && !shared.verification);
            assert_eq!(shared.expects_changes, task.expects_file_changes());
            let isolated = TaskAccess::for_task(&task, false);
            assert_eq!(isolated.read_only, task.is_read_only());
            assert_eq!(isolated.verification, task.is_verification());
        }
    }
}

#[test]
fn isolated_read_only_remains_read_only() {
    assert!(TaskAccess::for_task(&SubTask::new("review", "Inspect code"), false).read_only);
}
