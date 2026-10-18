//! Collaboration remains available inside mux without bypassing mutation leases.

use serde_json::json;

#[tokio::test]
async fn mux_allows_legacy_child_creation_and_messaging() {
    for action in [
        "ask",
        "spawn",
        "message",
        "list",
        "status",
        "interrupt",
        "close",
    ] {
        let input = json!({ "action": action });
        assert!(super::blocked_in_mux("agent", &input).await.is_none());
    }
}

#[tokio::test]
async fn mux_allows_first_class_collaboration_tools() {
    for tool in [
        "spawn_agent",
        "send_input",
        "followup_task",
        "send_message",
        "wait_agent",
        "list_agents",
        "resume_agent",
        "close_agent",
        "interrupt_agent",
    ] {
        assert!(super::blocked_in_mux(tool, &json!({})).await.is_none());
    }
}

#[tokio::test]
async fn mux_mutations_still_require_trusted_runtime_context() {
    for (tool, input) in [
        ("write", json!({ "path": "file.rs" })),
        ("exec_command", json!({ "cmd": "cargo fmt" })),
        ("tetherscript_plugin", json!({})),
    ] {
        let (_, success, metadata) = super::blocked_in_mux(tool, &input).await.unwrap();
        assert!(!success);
        assert_eq!(
            metadata.unwrap()["error_code"],
            "WORKTREE_COORDINATOR_UNAVAILABLE"
        );
    }
}
