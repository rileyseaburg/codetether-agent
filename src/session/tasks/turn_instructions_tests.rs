//! Regression coverage for the shared turn-boundary goal contract.

use super::{INSTRUCTIONS, append};

#[test]
fn goal_contract_is_appended_once_without_losing_existing_rules() {
    let prompt = append("Existing project instructions".into());
    assert!(prompt.starts_with("Existing project instructions"));
    assert!(prompt.contains(INSTRUCTIONS.trim()));
    assert_eq!(append(prompt.clone()), prompt);
}

#[test]
fn existing_goal_heading_does_not_suppress_the_full_contract() {
    let base = "## Session Goal Maintenance — Every Turn\nCustom project rules";
    let prompt = append(base.into());
    assert!(prompt.starts_with(base));
    assert!(prompt.contains(INSTRUCTIONS.trim()));
}

#[test]
fn contract_requires_goal_and_progress_at_every_turn_boundary() {
    for required in [
        "At the start of each turn",
        "`session_task` action `list`",
        "`session_task` action `set_goal`",
        "Before ending every turn",
        "ensure the full current goal is set or retained",
        "`session_task` action `reaffirm`",
        "`progress_note`",
        "all unfinished deliverables active",
        "independent verifier's decision",
        "reserve `create_goal` and budget changes for explicit user requests",
        "in your own session, not the parent's goal",
        "Never claim an update was saved without a successful tool result",
    ] {
        assert!(INSTRUCTIONS.contains(required), "missing: {required}");
    }
}
