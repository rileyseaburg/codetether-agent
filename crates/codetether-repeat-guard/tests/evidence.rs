mod support;
use support::*;
#[test]
fn calls_without_results_and_successful_edits_are_not_failed_progress() {
    let mut ledger = Ledger::default();
    ledger.observe_message(&call("pending", "bash", json!({"command":"echo ok"})), 0);
    assert_eq!(ledger.observations, 0);
    for i in 0..8 {
        outcome(
            &mut ledger,
            i,
            "edit",
            json!({"path":"a","new_string":"x"}),
            true,
            "applied",
        );
    }
    assert!(
        ledger
            .blocked_edit("edit", &json!({"path":"a","new_string":"x"}))
            .is_none()
    );
}
#[test]
fn failed_edit_repetition_survives_restart_and_requires_actual_new_evidence() {
    let input = json!({"path":"a","new_string":"x"});
    let mut ledger = Ledger::default();
    for i in 0..3 {
        outcome(
            &mut ledger,
            i,
            "edit",
            input.clone(),
            false,
            "target not found",
        );
        ledger = serde_json::from_str(&serde_json::to_string(&ledger).unwrap()).unwrap();
    }
    assert!(ledger.blocked_edit("edit", &input).is_some());
    outcome(
        &mut ledger,
        9,
        "edit",
        input.clone(),
        false,
        "REPEATED_EDIT_BLOCKED: same failure",
    );
    assert!(ledger.blocked_edit("edit", &input).is_some());
    assert!(
        ledger
            .blocked_edit("edit", &json!({"path":"a","new_string":"changed"}))
            .is_none()
    );
    outcome(
        &mut ledger,
        10,
        "read",
        json!({"path":"a"}),
        true,
        "new source contents",
    );
    assert!(ledger.blocked_edit("edit", &input).is_none());
}
#[test]
fn read_only_investigation_and_legitimate_waiting_do_not_trigger_write_nudges() {
    let mut ledger = Ledger::default();
    for i in 0..40 {
        outcome(
            &mut ledger,
            i,
            "read",
            json!({"path":format!("file{i}")}),
            true,
            "contents",
        );
    }
    for i in 40..80 {
        outcome(
            &mut ledger,
            i,
            "write_stdin",
            json!({"session_id":1}),
            true,
            "still running",
        );
    }
    assert!(ledger.take_nudge(15).is_none());
    assert_eq!(ledger.novel_observations, 40);
}
#[test]
fn arbitrary_bash_calls_and_repeated_cycles_are_not_new_progress() {
    let mut ledger = Ledger::default();
    for i in 0..40 {
        outcome(
            &mut ledger,
            i,
            "bash",
            json!({"command":"check"}),
            true,
            if i % 2 == 0 { "same A" } else { "same B" },
        );
    }
    assert_eq!(ledger.novel_observations, 2);
    assert!(
        ledger
            .take_nudge(15)
            .unwrap()
            .contains("do not end the task")
    );
    assert!(
        ledger
            .blocked_edit("bash", &json!({"command":"check"}))
            .is_none()
    );
}
#[test]
fn runtime_continuation_does_not_clear_failures_but_human_direction_does() {
    let mut ledger = Ledger::default();
    for i in 0..3 {
        outcome(&mut ledger, i, "edit", json!({}), false, "failure");
    }
    ledger.human_input(&text(
        "Continue this live CodeTether run from its structured checkpoint.\n...",
    ));
    assert!(ledger.blocked_edit("edit", &json!({})).is_some());
    ledger.human_input(&text("The target was repaired externally; try again"));
    assert!(ledger.blocked_edit("edit", &json!({})).is_none());
}
#[test]
fn pending_calls_survive_reload_and_raw_status_is_not_invented() {
    let mut ledger = Ledger::default();
    ledger.observe_message(&call("id", "edit", json!({"secret":"not persisted"})), 1);
    let serialized = serde_json::to_string(&ledger).unwrap();
    assert!(!serialized.contains("not persisted"));
    ledger = serde_json::from_str(&serialized).unwrap();
    ledger.observe_outcome("id", "failure", false, 4_000);
    ledger.observe_message(&reply("id", "failure"), 4_000);
    assert_eq!(ledger.observations, 1);
    for i in 2..8 {
        ledger.observe_message(&call(&i.to_string(), "edit", json!({})), i);
        ledger.observe_message(
            &reply(&i.to_string(), "Error: raw text without runtime status"),
            i + 1,
        );
    }
    assert!(ledger.blocked_edit("edit", &json!({})).is_none());
}

#[test]
fn changed_display_summaries_do_not_replace_raw_result_evidence() {
    let mut ledger = Ledger::default();
    for i in 0..20 {
        let id = i.to_string();
        ledger.observe_message(&call(&id, "read", json!({"path":"a"})), i * 2);
        ledger.observe_outcome(&id, "unchanged raw result", true, i * 2 + 1);
        ledger.observe_message(
            &reply(&id, &format!("different routed summary {i}")),
            i * 2 + 1,
        );
    }
    assert_eq!(ledger.observations, 20);
    assert_eq!(ledger.novel_observations, 1);
    assert!(ledger.take_nudge(15).is_some());
}

#[test]
fn a_repeated_failed_wait_is_not_treated_as_legitimate_waiting() {
    let mut ledger = Ledger::default();
    for i in 0..20 {
        outcome(
            &mut ledger,
            i,
            "write_stdin",
            json!({"session_id":99}),
            false,
            "unknown process",
        );
    }
    assert!(ledger.take_nudge(15).is_some());
}

#[test]
fn explicit_runtime_output_is_hashed_whole_even_when_it_contains_a_feedback_header() {
    let mut ledger = Ledger::default();
    for (i, label) in ["first", "second"].iter().enumerate() {
        let id = i.to_string();
        ledger.observe_message(&call(&id, "read", json!({"path":"log"})), i * 2);
        ledger.observe_outcome(
            &id,
            &format!("Tool call feedback\n- label: {label}\n\nOutput:\nsame body"),
            true,
            i * 2 + 1,
        );
    }
    assert_eq!(ledger.novel_observations, 2);
}
