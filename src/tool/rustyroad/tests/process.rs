//! Mocked MCP roundtrip and subprocess cleanup coverage.

use super::super::{invocation, params::Params, process::Process, session};
use serde_json::json;

#[cfg(unix)]
#[tokio::test]
async fn rustyroad_mock_roundtrip_preserves_success_and_errors_and_reaps_child() {
    let project = tempfile::tempdir().unwrap();
    for (name, expected) in [("rustyroad_config", true), ("rustyroad_error", false)] {
        let mut command = tokio::process::Command::new("sh");
        command.arg(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/rustyroad_mcp.sh"
        ));
        let mut process = Process::spawn_command(command).await.unwrap();
        let mut params: Params = serde_json::from_value(json!({"action":"call_tool",
            "cwd":project.path(), "environment":"test", "tool_name":name}))
        .unwrap();
        invocation::prepare(&mut params).unwrap();
        let result = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            session::execute(&mut process.rpc, &params),
        )
        .await
        .unwrap()
        .unwrap();
        process.close().await.unwrap();
        assert!(process.child.try_wait().unwrap().is_some());
        assert_eq!(result.success, expected);
        assert_eq!(result.metadata["rustyroad_version"], "fixture");
        assert!(result.output.starts_with("fixture"));
    }
}
