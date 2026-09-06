//! Status stays on the pure logical-state path and never samples the desktop.
use super::super::status;
use super::*;

#[test]
fn status_has_no_physical_observation_or_queued_messages() {
    let request = input(json!({"action":"status","hwnd":1}));
    let state = State {
        buttons: 1,
        ..State::default()
    };
    let result = status::reply(&request, state).unwrap();
    let output: Value = serde_json::from_str(&result.output).unwrap();
    assert!(result.success);
    assert!(output["observation"].is_null());
    assert_eq!(output["logical_state"]["buttons"], 1);
    assert_eq!(output["messages_queued"], 0);
    assert_eq!(output["application_effect_unverified"], true);
}
#[test]
fn action_requests_do_not_take_status_shortcut() {
    let request = input(json!({"action":"click","hwnd":1,"x":0,"y":0}));
    assert!(status::reply(&request, State::default()).is_none());
}
