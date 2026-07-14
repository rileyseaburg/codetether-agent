use crate::worktree::MergeResult;

pub(super) fn noop(branch: &str) -> MergeResult {
    create(
        true,
        vec![],
        0,
        format!("Branch '{branch}' had no changes to merge"),
    )
}

pub(super) fn conflict(conflicts: Vec<String>) -> MergeResult {
    create(
        false,
        conflicts,
        0,
        "Merge has conflicts that need resolution".into(),
    )
}

pub(super) fn success(branch: &str, commit: String, files: usize) -> MergeResult {
    create(
        true,
        vec![],
        files,
        format!("Verified merge of '{branch}' ({commit})"),
    )
}

fn create(
    success: bool,
    conflicts: Vec<String>,
    files_changed: usize,
    summary: String,
) -> MergeResult {
    MergeResult {
        success,
        aborted: false,
        conflicts,
        conflict_diffs: vec![],
        files_changed,
        summary,
    }
}
