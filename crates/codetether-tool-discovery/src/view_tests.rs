//! Existing discovery-view serialization regression.
use crate::{RegisteredTool, RegisteredToolView};
use serde_json::json;

#[test]
fn registered_tool_view_derives_discovery_contract() {
    let now = chrono::Utc::now();
    let tool = RegisteredTool {
        id: "demo".into(),
        name: "Demo".into(),
        description: "Demo tool".into(),
        version: "1.0.0".into(),
        endpoint: "https://example.test/tool".into(),
        capabilities: vec!["demo".into()],
        parameters: json!({}),
        execution_mode: crate::contract::DISCOVERY_ONLY_MODE.into(),
        registered_at: now,
        last_heartbeat: now,
        expires_at: now,
    };

    let value = serde_json::to_value(RegisteredToolView::from(tool)).unwrap();

    assert_eq!(value["execution_mode"], "discovery_only");
    assert_eq!(value["agent_executable"], false);
}
