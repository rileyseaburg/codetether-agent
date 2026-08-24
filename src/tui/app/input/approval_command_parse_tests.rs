//! Explicit approval target parsing tests.

use super::intent::ApprovalIntent;

#[test]
fn explicit_id_can_select_session_authority() {
    let action = super::parse::Action::parse("/approve approval-1 session").expect("action");
    assert_eq!(action.id, Some("approval-1"));
    assert_eq!(action.intent, ApprovalIntent::ApproveForSession);
}

#[test]
fn explicit_id_can_select_once_authority() {
    let action = super::parse::Action::parse("/approve approval-1 once").expect("action");
    assert_eq!(action.id, Some("approval-1"));
    assert_eq!(action.intent, ApprovalIntent::ApproveOnce);
}
