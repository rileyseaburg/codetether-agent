use super::{Args, ListAgentsTool};
use crate::tool::Tool;
use serde_json::json;

#[test]
fn schema_exposes_optional_path_prefix() {
    let schema = ListAgentsTool.parameters();
    assert!(schema["properties"].get("path_prefix").is_some());
}

#[test]
fn parses_relative_prefix_and_runtime_context() {
    let args: Args = serde_json::from_value(json!({
        "path_prefix":"worker", "__ct_session_id":"parent"
    }))
    .unwrap();
    assert_eq!(args.path_prefix.as_deref(), Some("worker"));
    assert_eq!(args.context.session_id.as_deref(), Some("parent"));
}
