//! Editing directives after introductory clauses must retain write capability.
use crate::SubTask;

#[test]
fn introduced_edit_wins_over_later_readback_and_readonly_metadata() {
    for separator in [",", ";", ".", "\n"] {
        let instruction = format!(
            "In /tmp/codetether-swarm-edit-szCtWj{separator} edit ONLY alpha.txt: \
             replace 'alpha: pending' with 'alpha: edited-by-swarm' using an editing tool. \
             Then read alpha.txt back and confirm it. Do not run Python."
        );
        let task = SubTask::new("alpha-editor", &instruction).with_needs_worktree(false);
        assert!(task.needs_worktree(), "{instruction}");
        assert!(!task.is_read_only(), "{instruction}");
        assert!(task.expects_file_changes(), "{instruction}");
    }
}

#[test]
fn introductory_clause_does_not_turn_negated_edits_into_mutation() {
    let task = SubTask::new(
        "review",
        "In this repository, do not edit files. Then read the API.",
    );
    assert!(task.is_read_only());
    assert!(!task.expects_file_changes());
}
