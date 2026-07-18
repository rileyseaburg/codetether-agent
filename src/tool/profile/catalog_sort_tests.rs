//! Deterministic schema ordering tests.

use super::super::sort;
use crate::provider::ToolDefinition;

#[test]
fn schema_order_is_deterministic() {
    let definitions = ["read", "apply_patch", "bash"]
        .into_iter()
        .map(|name| ToolDefinition {
            name: name.into(),
            description: String::new(),
            parameters: serde_json::json!({}),
        })
        .collect();
    let names: Vec<_> = sort(definitions)
        .into_iter()
        .map(|tool| tool.name)
        .collect();
    assert_eq!(names, ["apply_patch", "bash", "read"]);
}
