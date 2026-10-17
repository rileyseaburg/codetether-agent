use crate::effective_error;

#[test]
fn blocked_and_pending_are_failures() {
    assert!(effective_error("evidence\nSTATUS: blocked", None).is_some());
    assert!(effective_error("next action\nSTATUS: pending", None).is_some());
    assert!(effective_error("evidence\nSTATUS: completed", None).is_none());
}

#[test]
fn missing_footer_preserves_useful_child_output() {
    assert_eq!(
        effective_error("Review findings: two routing defects.", None),
        None
    );
}

#[test]
fn runtime_error_is_never_replaced_by_deliverable_metadata() {
    for response in [
        "Review findings",
        "STATUS: completed",
        "STATUS: blocked",
        "",
    ] {
        assert_eq!(
            effective_error(response, Some("expired provider credentials".into())),
            Some("expired provider credentials".into())
        );
    }
}
