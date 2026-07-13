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
