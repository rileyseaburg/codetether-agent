use serde_json::json;

use crate::tool::Tool;
use crate::tool::tetherscript::TetherScriptPluginTool;

#[tokio::test]
async fn builds_gpt_5_6_sol_request_via_runtime() {
    let result = TetherScriptPluginTool::new()
        .execute(json!({
            "path": "examples/tetherscript/openai_codex_5_6_sol.tether",
            "hook": "request_body",
            "args": ["Inspect the workspace"]
        }))
        .await
        .unwrap();

    assert!(result.success, "hook failed: {}", result.output);
    let body = result.metadata["value"].as_str().unwrap();
    let body: serde_json::Value = serde_json::from_str(body).unwrap();
    assert_eq!(body["model"], "gpt-5.6-sol");
    assert_eq!(
        body["input"][0]["content"][0]["text"],
        "Inspect the workspace"
    );
    assert_eq!(body["reasoning"]["effort"], "medium");
}
