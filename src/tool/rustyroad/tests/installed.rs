//! Opt-in smoke coverage against the real installed RustyRoad backend.

use super::super::RustyRoadTool;
use super::smoke_helpers;
use crate::{
    approval::test_env::{ScopedEnv, lock_env},
    config::AccessMode,
    tool::{Tool, ToolRegistry},
};
use serde_json::{Value, json};

#[test]
fn rustyroad_is_registered_in_standard_registries() {
    let registry = ToolRegistry::with_defaults();
    assert!(registry.get("rustyroad").is_some());
}

#[tokio::test]
#[ignore = "requires rustyroad-mcp on PATH; config only, no database connection"]
async fn rustyroad_installed_discovery_and_config_smoke() {
    let _lock = lock_env();
    let project = smoke_helpers::project();
    let _env = ScopedEnv::data_dir_with_access(project.path(), AccessMode::Full);
    let before = (
        std::env::current_dir().unwrap(),
        std::env::var_os("ENVIRONMENT"),
    );
    let tools = RustyRoadTool
        .execute(json!({"action":"list_tools", "cwd":project.path()}))
        .await
        .unwrap();
    assert!(tools.success, "{}", tools.output);
    let tools: Vec<Value> = serde_json::from_str(&tools.output).unwrap();
    assert!(tools.iter().any(|tool| tool["name"] == "rustyroad_config"));
    for (environment, database) in [("dev", "dev_fixture"), ("test", "test_fixture")] {
        let result = RustyRoadTool
            .execute(json!({"action":"call_tool", "cwd":project.path(),
            "environment":environment, "tool_name":"rustyroad_config"}))
            .await
            .unwrap();
        smoke_helpers::assert_config(&result, project.path(), environment, database);
    }
    assert_eq!(
        before,
        (
            std::env::current_dir().unwrap(),
            std::env::var_os("ENVIRONMENT")
        )
    );
}