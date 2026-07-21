use super::SpawnAgentTool;
use crate::tool::Tool;

#[test]
fn v2_schema_requires_canonical_task_name_and_message() {
    let schema = SpawnAgentTool.parameters();
    assert_eq!(
        schema["required"],
        serde_json::json!(["task_name", "message"])
    );
    assert!(schema["properties"].get("fork_context").is_none());
    assert!(schema["properties"].get("model").is_none());
}
