//! Runtime tests for the bash_guard TetherScript plugin.

use serde_json::{Value, json};

use crate::tool::Tool;
use crate::tool::tetherscript::TetherScriptPluginTool;

async fn run_guarded(command: &str, timeout_ms: u64) -> Value {
    let tool = TetherScriptPluginTool::new();
    let result = tool
        .execute(json!({
            "path": "examples/tetherscript/bash_guard.tether",
            "hook": "run_guarded",
            "args": [command, timeout_ms]
        }))
        .await
        .unwrap();
    assert!(result.success, "plugin failed: {}", result.output);
    let encoded = result.metadata["value"]["ok"].as_str().unwrap();
    serde_json::from_str(encoded).unwrap()
}

#[tokio::test]
async fn run_guarded_returns_output_for_fast_command() {
    let value = run_guarded("echo guarded", 5000).await;
    assert_eq!(value["success"], json!(true));
    assert_eq!(value["timed_out"], json!(false));
    assert_eq!(value["stdout"].as_str().unwrap().trim(), "guarded");
}

#[tokio::test]
async fn run_guarded_kills_hung_command_at_timeout() {
    let value = run_guarded("sleep 30", 300).await;
    assert_eq!(value["success"], json!(false));
    assert_eq!(value["timed_out"], json!(true));
}
