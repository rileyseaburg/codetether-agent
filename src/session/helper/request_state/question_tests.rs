//! Interactive schemas are available only inside the foreground UI turn.
use super::*;

#[tokio::test]
async fn question_advertisement_requires_attached_frontend() {
    let registry = ToolRegistry::with_defaults();
    let has_question = |tools: &[ToolDefinition]| tools.iter().any(|t| t.name == "question");
    let headless = active_tool_definitions(&registry, "openai-codex");
    assert!(!has_question(&headless));
    codetether_question::runtime::scope("question-schema-test".into(), async {
        let active = active_tool_definitions(&registry, "openai-codex");
        assert!(has_question(&active));
        let advertised = advertised_tools(true, &active, "openai-codex");
        assert!(has_question(&advertised));
    })
    .await;
    assert!(!has_question(&active_tool_definitions(
        &registry,
        "openai-codex"
    )));
}
