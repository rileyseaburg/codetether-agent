//! Await an exec-command fixture without assuming its first poll is final.

use super::Registry;
use crate::tool::{Tool, ToolResult, exec_command::ExecCommandTool, write_stdin::WriteStdinTool};
use serde_json::{Value, json};
use std::sync::Arc;
use tokio::time::{Duration, timeout};

pub(super) async fn completed(
    tool: ExecCommandTool,
    sessions: Arc<Registry>,
    input: Value,
) -> ToolResult {
    timeout(Duration::from_secs(5), async {
        let mut result = tool.execute(input).await.unwrap();
        let mut output = result.output.clone();
        let poller = WriteStdinTool::new(sessions);
        while result.metadata["running"] == json!(true) {
            let id = result.metadata["session_id"].as_u64().unwrap();
            result = poller
                .execute(json!({"session_id": id, "chars": "", "yield_time_ms": 250}))
                .await
                .unwrap();
            output.push_str(&result.output);
        }
        result.output = output;
        result
    })
    .await
    .expect("command fixture did not finish within five seconds")
}

#[tokio::test]
async fn completion_fixture_collects_late_output() {
    let sessions = Arc::new(Registry::default());
    let tool = ExecCommandTool::new(sessions.clone(), None);
    let result = completed(
        tool,
        sessions,
        json!({"cmd": "sleep 0.4; printf delayed-output", "yield_time_ms": 250}),
    )
    .await;
    assert!(result.success);
    assert_eq!(result.metadata["exit_code"], json!(0));
    assert!(result.output.contains("delayed-output"));
}
