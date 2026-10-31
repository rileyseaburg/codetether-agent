//! Mentions and prohibitions of refactoring remain read-only tasks.
use crate::SubTask;

#[test]
fn refactoring_context_and_negated_directives_remain_read_only() {
    for instruction in [
        "Review generated artifacts and extracted controller code.",
        "Inspect split results. Analyze update logs.",
        "Review the migration. Do not extract any modules.",
        "Review the migration. Do not split any modules.",
        "Review the migration. Do not replace inline SQL.",
        "Review the migration. Do not generate artifacts.",
        "Review the migration. Do not wire the query into the controller.",
        "Review the migration. Do not update the controller.",
        "Review the migration. Do not migrate any queries.",
    ] {
        let task = SubTask::new("controller_zone", instruction).with_needs_worktree(false);
        assert!(task.is_read_only(), "{instruction}");
        assert!(!task.expects_file_changes(), "{instruction}");
    }
}
