use crate::support::state;
use codetether_gemini_web_protocol::images::Continuation;
use serde_json::json;

#[test]
fn continuation_supports_node_files_and_snake_case_aliases() {
    let expected = state();
    let node = json!({"conversationId": "c_test", "responseId": "r_test", "choiceId": "rc_test"});
    let rust =
        json!({"conversation_id": "c_test", "response_id": "r_test", "choice_id": "rc_test"});
    assert_eq!(serde_json::to_value(&expected).unwrap(), node);
    for value in [node, rust] {
        let parsed: Continuation = serde_json::from_value(value).unwrap();
        assert_eq!(parsed, expected);
        parsed.validate().unwrap();
    }
}

#[test]
fn continuation_checks_each_prefix_ascii_and_suffix_boundaries() {
    for invalid in ["", "c_", "r_wrong", "c_雪", "c_a\n", "https://secret.test/"] {
        let mut value = state();
        value.conversation_id = invalid.into();
        assert_eq!(
            value.validate().unwrap_err().to_string(),
            "image_state_invalid"
        );
    }
    let mut value = state();
    value.conversation_id = format!("c_{}", "x".repeat(128));
    value.validate().unwrap();
    value.conversation_id.push('x');
    assert!(value.validate().is_err());
    value = state();
    value.response_id = "c_not_response".into();
    assert!(value.validate().is_err());
    value = state();
    value.choice_id = "r_not_choice".into();
    assert!(value.validate().is_err());
    value = state();
    value.choice_id = "rc_A-z_09".into();
    value.validate().unwrap();
}
