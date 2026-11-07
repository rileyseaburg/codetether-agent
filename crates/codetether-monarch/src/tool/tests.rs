//! Fail-closed input and remote capability tests.
use super::{catalog::readable, input::parse};
use serde_json::json;

#[test]
fn fail_closed_tool_classification() {
    assert!(!readable(&json!({"name":"get_accounts"})));
    assert!(!readable(&json!({"annotations":{"readOnlyHint":false}})));
    assert!(readable(&json!({"annotations":{"readOnlyHint":true}})));
}

#[test]
fn refresh_uses_existing_grant_without_accepting_new_authority() {
    use super::input::Input;
    assert!(matches!(
        parse(json!({"action":"refresh"})),
        Ok(Input::Refresh {})
    ));
    assert!(matches!(
        parse(json!({"action":"refresh","__ct_session_id":"fixture"})),
        Ok(Input::Refresh {})
    ));
    for key in [
        "token",
        "refresh_token",
        "endpoint",
        "scope",
        "write",
        "force",
    ] {
        let mut input = json!({"action":"refresh"});
        input[key] = json!("fixture");
        assert!(parse(input).is_err());
    }
    use codetether_tool_core::Tool;
    let schema = super::MonarchTool.parameters();
    assert!(
        schema["properties"]["action"]["enum"]
            .as_array()
            .unwrap()
            .contains(&json!("refresh"))
    );
}

#[test]
fn no_auth_or_endpoint_override_from_tool_arguments() {
    for input in [
        json!({"action":"login"}),
        json!({"action":"status","token":"x"}),
        json!({"action":"list_tools","endpoint":"https://evil.test"}),
    ] {
        assert!(parse(input).is_err());
    }
}
