//! Refactoring work must retain editing capabilities through review steps.
use crate::SubTask;

#[test]
fn refactoring_directives_override_read_only_metadata_and_followup_inspection() {
    for instruction in [
        "Extract controller_zone into a dedicated crate. Then inspect the diff.",
        "Split the controller into focused modules. Then review the result.",
        "Replace inline SQL with a RustyRoad artifact. Then inspect the diff.",
        "Generate the RustyRoad query artifact. Then review the result.",
        "Wire the generated query into the controller. Then inspect the diff.",
        "Update the controller to use the query artifact. Then review the result.",
        "Migrate the controller to the generated query. Then inspect the diff.",
        "Inspect split results and update logs.",
    ] {
        for preference in [None, Some(false), Some(true)] {
            let mut task = SubTask::new("controller_zone", instruction);
            task.needs_worktree = preference;
            assert!(task.needs_worktree(), "{instruction} ({preference:?})");
            assert!(!task.is_read_only(), "{instruction} ({preference:?})");
            assert!(
                task.expects_file_changes(),
                "{instruction} ({preference:?})"
            );
            assert!(!task.is_verification(), "{instruction} ({preference:?})");
        }
    }
}
