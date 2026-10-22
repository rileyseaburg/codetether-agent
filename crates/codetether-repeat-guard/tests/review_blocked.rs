mod support;
use support::*;
#[test]
fn guard_rejections_trigger_review_without_reopening_failed_edits() {
    let mut ledger = Ledger::default();
    for i in 0..3 {
        outcome(&mut ledger, i, "edit", json!({}), false, "target absent");
    }
    ledger.take_review(3).unwrap();
    for i in 3..18 {
        outcome(
            &mut ledger,
            i,
            "edit",
            json!({}),
            false,
            "REPEATED_EDIT_BLOCKED: unchanged failure",
        );
    }
    assert!(ledger.blocked_edit("edit", &json!({})).is_some());
    assert!(ledger.take_review(15).unwrap().contains("15 failures"));
    assert_eq!(ledger.novel_observations, 1);
}
