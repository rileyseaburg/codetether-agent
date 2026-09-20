//! Opt-in credential-free public HTTPS probe through the embedded plugin tool.
//!
//! This is a new test process, not activation evidence for an existing TUI.

#![cfg(feature = "tetherscript")]

use codetether_agent::tool::{Tool, tetherscript::TetherScriptPluginTool};
use serde_json::json;

#[tokio::test]
#[ignore = "requires public HTTPS access and native platform trust roots"]
async fn native_plugin_public_https_probe() {
    let result = TetherScriptPluginTool::new()
        .execute(json!({
            "source": r#"
fn probe() {
    let response = http_request("HEAD", "https://example.com/", "", map())?
    return response["status"]
}
"#,
            "hook": "probe",
            "timeout_secs": 20
        }))
        .await
        .expect("native plugin execution should return a tool result");
    assert!(result.success, "native HTTPS probe: {}", result.output);
    assert_eq!(result.metadata.get("value"), Some(&json!(200)));
    tracing::info!(
        pid = std::process::id(),
        status = 200,
        "Native plugin HTTPS probe"
    );
}
