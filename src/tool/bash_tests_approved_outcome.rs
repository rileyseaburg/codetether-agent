use crate::tool::{Tool, ToolResult, bash::BashTool};
use serde_json::{Value, json};

pub(super) async fn verify(tool: &BashTool, replay_args: Value, result: ToolResult) {
    if crate::tool::sandbox::unavailable_reason().is_some()
        && !crate::tool::sandbox::direct_fallback_env_allowed()
    {
        assert!(!result.success);
        assert!(result.output.contains("sandbox preflight failed"));
        let retry = tool.execute(replay_args).await.unwrap();
        assert!(!retry.output.contains("approval claim failed"));
        return;
    }
    assert!(result.success, "{}", result.output);
    if crate::tool::sandbox::unavailable_reason().is_some() {
        assert_eq!(result.metadata.get("sandboxed"), Some(&json!(false)));
        assert_eq!(
            result.metadata.get("unsafe_fallback_reason"),
            Some(&json!("approved_os_sandbox_unavailable_fallback"))
        );
    } else {
        assert_eq!(result.metadata.get("sandboxed"), Some(&json!(true)));
    }
    let replay = tool.execute(replay_args).await.unwrap();
    assert!(!replay.success);
}