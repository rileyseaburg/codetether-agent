//! Render cleanup and return-to-branch notices.

pub(crate) fn render(
    cleanup_count: usize,
    returned_to_original: bool,
    starting_branch: &Option<String>,
) -> (String, String) {
    let cleanup_note = if cleanup_count > 0 {
        format!(
            "\n\n*(Cleaned up {} orphaned worktree(s)/branch(es))*",
            cleanup_count
        )
    } else {
        String::new()
    };

    let branch_note = if returned_to_original {
        format!(
            "\n*(Returned to branch: {})*",
            starting_branch.as_deref().unwrap_or("main")
        )
    } else {
        String::new()
    };

    (cleanup_note, branch_note)
}
