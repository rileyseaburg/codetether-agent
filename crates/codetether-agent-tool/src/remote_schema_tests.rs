//! Remote conversation fields retain their existing schema contract.
use super::agent_tool_parameters;

#[test]
fn remote_context_schema_matches_session_safety_contract() {
    let schema = agent_tool_parameters();
    let context = &schema["properties"]["context_id"];

    assert_eq!(context["maxLength"], 128);
    assert_eq!(context["pattern"], "^[A-Za-z0-9_-]+$");
}

#[test]
fn remote_messages_accept_a_conversation_context() {
    let schema = agent_tool_parameters();
    let context = &schema["properties"]["context_id"];

    assert_eq!(context["type"], "string");
    assert!(
        context["description"]
            .as_str()
            .unwrap()
            .contains("conversation")
    );
}
