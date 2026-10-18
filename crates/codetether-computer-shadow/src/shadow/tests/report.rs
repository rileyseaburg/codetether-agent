//! Queue reporting must distinguish unknown state and unverified app effects.
use super::super::{replay, report};
use super::*;

#[test]
fn preflight_failure_does_not_claim_state_was_cleared() {
    let result = report::failure(anyhow::anyhow!("invalid HWND"));
    let output: Value = serde_json::from_str(&result.output).unwrap();
    assert!(!result.success);
    assert!(output["logical_state"].is_null());
    assert_eq!(output["messages_queued"], 0);
    assert_eq!(output["application_effect_unverified"], true);
}
#[test]
fn successful_queue_does_not_confirm_application_effect() {
    let plan = planned(json!({"action":"click","hwnd":1,"x":0,"y":0}));
    let mut state = State::default();
    let outcome = replay::execute(&plan, &mut state, |_| Ok(()));
    let result = report::result(Some(1), state, outcome, Value::Null);
    let output: Value = serde_json::from_str(&result.output).unwrap();
    assert!(result.success);
    assert_eq!(output["messages_queued"], 3);
    assert_eq!(output["application_effect_unverified"], true);
}
#[test]
fn first_post_failure_preserves_original_state() {
    let plan = planned(json!({"action":"mouse_down","hwnd":1,"x":0,"y":0}));
    let mut state = State::default();
    let outcome = replay::execute(&plan, &mut state, |_| anyhow::bail!("queue denied"));
    assert_eq!(outcome.queued, 0);
    assert_eq!(state, State::default());
}
