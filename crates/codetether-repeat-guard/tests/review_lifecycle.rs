mod support;
use support::*;
#[test]
fn waits_and_proposed_calls_do_not_count_but_failed_waits_do() {
    let mut ledger = Ledger::default();
    for i in 0..40 {
        outcome(
            &mut ledger,
            i,
            "write_stdin",
            json!({"session_id":1}),
            true,
            "running",
        );
        ledger.observe_message(&call("pending", "read", json!({})), i);
    }
    assert!(ledger.take_review(1).is_none());
    outcome(
        &mut ledger,
        40,
        "write_stdin",
        json!({"session_id":1}),
        false,
        "unknown process",
    );
    assert!(ledger.take_review(1).unwrap().contains("1 failures"));
    assert!(ledger.take_review(1).is_none());
}
#[test]
fn trusted_human_direction_resets_review_and_legacy_state_loads() {
    let mut ledger: Ledger = serde_json::from_str("{\"observations\":900}").unwrap();
    outcome(&mut ledger, 1, "read", json!({}), true, "source");
    assert!(ledger.take_review(0).is_none());
    ledger.human_input(&text("Now investigate the other component"));
    assert!(ledger.take_review(1).is_none());
    assert_eq!(ledger.observations, 901);
}
