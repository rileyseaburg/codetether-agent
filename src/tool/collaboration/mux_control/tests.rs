use crate::tool::Tool;

#[test]
fn schema_is_mux_only_and_supports_watch() {
    let schema = super::MuxControlTool.parameters();
    let actions = schema["properties"]["action"]["enum"].as_array().unwrap();
    assert!(actions.iter().any(|item| item == "watch"));
    assert!(actions.iter().any(|item| item == "roll"));
    assert!(!actions.iter().any(|item| item == "spawn"));
}
