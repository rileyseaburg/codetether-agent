//! Injected posting failures retain exact queue counts and held-state evidence.
use super::super::{replay, report};
use super::*;

#[test]
fn partial_mouse_failure_preserves_shadow_hold() {
    let plan = planned(json!({"action":"click","hwnd":1,"x":4,"y":5}));
    let mut state = State::default();
    let outcome = replay::execute(&plan, &mut state, |event| {
        anyhow::ensure!(event.message != 0x202, "mock UIPI denial");
        Ok(())
    });
    assert_eq!(outcome.queued, 2);
    assert_eq!(state.buttons, 1);
    let result = report::result(Some(1), state, outcome, Value::Null);
    assert!(!result.success);
    let output: Value = serde_json::from_str(&result.output).unwrap();
    assert_eq!(output["messages_queued"], 2);
    assert_eq!(output["application_effect_unverified"], true);
}
#[test]
fn partial_key_failure_can_only_release_shadow_key() {
    let plan = planned(json!({"action":"press_key","hwnd":1,"key":"Enter"}));
    let mut state = State::default();
    replay::execute(&plan, &mut state, |event| {
        anyhow::ensure!(event.message != 0x101, "mock failure");
        Ok(())
    });
    let stop = plan::build(
        &input(json!({"action":"stop","hwnd":1})),
        Geometry::default(),
        state,
    )
    .unwrap();
    assert_eq!(stop.events.len(), 1);
    assert_eq!(stop.events[0].message, 0x101);
    assert_eq!(stop.events[0].wparam, 0x0d);
    assert!(stop.state.pending_key.is_none());
}
