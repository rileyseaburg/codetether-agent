use super::*;

#[test]
fn read_only_definitions_hide_mutating_tools() {
    let tools = vec![
        ToolDefinition {
            name: "read".into(),
            description: "".into(),
            parameters: serde_json::json!({}),
        },
        ToolDefinition {
            name: "bash".into(),
            description: "".into(),
            parameters: serde_json::json!({}),
        },
    ];
    let names: Vec<String> = definitions(&tools, true)
        .into_iter()
        .map(|tool| tool.name)
        .collect();
    assert_eq!(names, vec!["read"]);
}

#[test]
fn read_only_registry_removes_mutating_tools() {
    let mut registry = ToolRegistry::with_defaults();
    restrict_registry(&mut registry, true);
    for denied in ["write", "edit", "multiedit", "patch", "bash", "ralph", "go"] {
        assert!(!registry.contains(denied), "{denied} should be blocked");
    }
    assert!(registry.contains("read"));
}
