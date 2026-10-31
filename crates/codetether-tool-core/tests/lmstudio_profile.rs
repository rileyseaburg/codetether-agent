//! LM Studio starts with a small executable-tool schema surface.
use codetether_provider_core::ToolDefinition;

#[test]
fn automatic_lmstudio_profile_keeps_basics_and_full_is_explicit() {
    // This integration-test process has only this one environment-mutating test.
    unsafe { std::env::remove_var("CODETETHER_TOOL_PROFILE") };
    let tools: Vec<_> = [
        "read",
        "get_goal",
        "session_task",
        "browserctl",
        "image_gen",
        "exec_command",
    ]
    .into_iter()
    .map(|name| ToolDefinition {
        name: name.into(),
        description: "fixture".into(),
        parameters: serde_json::json!({}),
    })
    .collect();
    let selected = codetether_tool_core::profile::apply_for_provider(tools.clone(), "lmstudio");
    let names: Vec<_> = selected.iter().map(|tool| tool.name.as_str()).collect();
    assert_eq!(names, ["exec_command", "get_goal", "read", "session_task"]);
    unsafe { std::env::set_var("CODETETHER_TOOL_PROFILE", "full") };
    assert_eq!(
        codetether_tool_core::profile::apply_for_provider(tools.clone(), "lmstudio").len(),
        tools.len()
    );
    unsafe { std::env::remove_var("CODETETHER_TOOL_PROFILE") };
}
