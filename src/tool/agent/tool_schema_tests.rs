use super::agent_tool_parameters;

#[test]
fn list_action_is_not_described_as_a_model_roster() {
    let schema = agent_tool_parameters();
    let description = schema["properties"]["action"]["description"]
        .as_str()
        .expect("action description should exist");

    assert!(description.contains("spawned agent instances"));
    assert!(description.contains("not available providers or models"));
    assert!(description.contains("codetether models --json"));
}

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
