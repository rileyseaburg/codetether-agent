//! Fail-closed input and remote capability tests.
use super::{catalog::readable, input::Input};
use serde_json::json;

#[test]
fn fail_closed_tool_classification() {
    assert!(!readable(&json!({"name":"get_accounts"})));
    assert!(!readable(&json!({"annotations":{"readOnlyHint":false}})));
    assert!(readable(&json!({"annotations":{"readOnlyHint":true}})));
}

#[test]
fn no_auth_or_endpoint_override_from_tool_arguments() {
    for input in [
        json!({"action":"login"}),
        json!({"action":"status","token":"x"}),
        json!({"action":"list_tools","endpoint":"https://evil.test"}),
    ] {
        assert!(serde_json::from_value::<Input>(input).is_err());
    }
}
