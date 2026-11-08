//! Web research is immediately advertised while specialists remain discoverable.
use codetether_provider_core::ToolDefinition;
use codetether_tool_catalog::{advertised_tools, promote};

#[test]
fn lmstudio_web_tools_are_primary_and_specialist_discovery_is_retained() {
    // This integration-test process has only this one environment-mutating test.
    unsafe { std::env::remove_var("CODETETHER_TOOL_PROFILE") };
    let tools: Vec<_> = ["websearch", "webfetch", "read", "image_gen"]
        .into_iter()
        .map(|name| ToolDefinition {
            name: name.into(),
            description: format!("{name} schema"),
            parameters: serde_json::json!({"type":"object","properties":{"query":{"type":"string"}}}),
        })
        .collect();
    let mut advertised = advertised_tools(true, &tools, "lmstudio", |tools| tools);
    let names: Vec<_> = advertised.iter().map(|tool| tool.name.as_str()).collect();
    assert_eq!(names, ["read", "webfetch", "websearch", "list_tools"]);
    for original in &tools[..2] {
        let actual = advertised
            .iter()
            .find(|tool| tool.name == original.name)
            .unwrap();
        assert_eq!(actual.parameters, original.parameters);
        assert_eq!(actual.description, original.description);
    }
    promote(
        &tools,
        &mut advertised,
        &serde_json::json!({"tool":"image_gen"}),
    );
    assert!(advertised.iter().any(|tool| tool.name == "image_gen"));
    assert!(advertised_tools(false, &tools, "lmstudio", |tools| tools).is_empty());
}
