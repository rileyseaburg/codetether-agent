mod support;
use support::*;
#[test]
fn bounded_records_and_pending_state_do_not_store_payloads() {
    let mut ledger = Ledger::default();
    for i in 0..1000 {
        outcome(
            &mut ledger,
            i,
            "read",
            json!({"path":i}),
            true,
            "private source text",
        );
    }
    for i in 1000..2000 {
        ledger.observe_message(
            &call(&i.to_string(), "read", json!({"secret":"private args"})),
            i,
        );
    }
    let value = serde_json::to_value(&ledger).unwrap();
    assert_eq!(value["records"].as_array().unwrap().len(), 128);
    assert_eq!(value["pending"].as_array().unwrap().len(), 128);
    let raw = value.to_string();
    assert!(!raw.contains("private source"));
    assert!(!raw.contains("private args"));
}
#[test]
fn meaningful_instruction_changes_are_distinct_but_approval_wording_is_not() {
    let mut ledger = Ledger::default();
    for i in 0..3 {
        outcome(
            &mut ledger,
            i,
            "edit",
            json!({"instruction":"first", "justification":format!("reason {i}")}),
            false,
            "failed",
        );
    }
    assert!(
        ledger
            .blocked_edit("edit", &json!({"instruction":"first"}))
            .is_some()
    );
    assert!(
        ledger
            .blocked_edit("edit", &json!({"instruction":"changed"}))
            .is_none()
    );
}
#[test]
fn timing_and_working_directory_feedback_do_not_create_new_observations() {
    let mut ledger = Ledger::default();
    for i in 0..20 {
        ledger.observe_message(
            &call(&i.to_string(), "bash", json!({"command":"true"})),
            i * 2,
        );
        ledger.observe_message(&reply(&i.to_string(), &format!("Tool call feedback\n- status: success\n- duration_ms: {i}\n- working_dir: /workspace\n\nOutput:\nunchanged")), i*2+1);
    }
    assert_eq!(ledger.novel_observations, 1);
    assert!(ledger.take_nudge(15).is_some());
}
