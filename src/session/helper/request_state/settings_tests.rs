use super::{model_supports_tools, system_prompt_for};
use crate::provider::ToolDefinition;

fn tool(name: &str) -> ToolDefinition {
    ToolDefinition {
        name: name.into(),
        description: format!("{name} description"),
        parameters: serde_json::json!({"type": "object"}),
    }
}

#[test]
fn gemini_web_advertises_compact_tools_with_discovery() {
    let tools = vec![tool("memory"), tool("exec_command"), tool("read")];
    let advertised = super::super::tools::advertised_tools(
        model_supports_tools("gemini-web", "gemini-web-fast"),
        &tools,
        "gemini-web",
    );
    let names: Vec<_> = advertised.iter().map(|tool| tool.name.as_str()).collect();
    assert_eq!(names, ["exec_command", "read", "list_tools"]);
}

#[test]
fn local_cuda_does_not_advertise_native_tools() {
    for provider in ["local-cuda", "local_cuda", "localcuda"] {
        assert!(!model_supports_tools(provider, "model"));
    }
}

#[test]
fn openrouter_safety_classifier_does_not_advertise_native_tools() {
    let model = "nvidia/nemotron-3.5-content-safety:free";
    assert!(!model_supports_tools("openrouter", model));
    let tools = vec![tool("agent")];
    assert!(super::super::tools::advertised_tools(false, &tools, "openrouter").is_empty());
}

#[test]
fn light_prompt_keeps_project_instructions() {
    let workspace = tempfile::tempdir().expect("workspace");
    std::fs::create_dir(workspace.path().join(".git")).expect("git marker");
    std::fs::write(
        workspace.path().join("AGENTS.md"),
        "LOCAL_PROMPT_PROJECT_RULE",
    )
    .expect("instructions");

    let prompt = system_prompt_for("local_cuda", false, &[], workspace.path(), false);
    assert!(prompt.contains("LOCAL_PROMPT_PROJECT_RULE"));
}
