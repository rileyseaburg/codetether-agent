//! Wait for all twenty workers, then execute real patches concurrently.
use codetether_swarm_core::agent_retry::{AgentLoopExit, AgentOutcome};
use codetether_swarm_tool::runtime::LoopInput;
use std::sync::LazyLock;
static READY: LazyLock<tokio::sync::Barrier> = LazyLock::new(|| tokio::sync::Barrier::new(20));
pub(super) async fn execute(input: LoopInput<super::registry::Catalog>) -> AgentOutcome {
    assert!(input.context.shared_workspace);
    assert!(!input.context.read_only && !input.context.verification);
    assert!(input.system.contains("SHARED WRITABLE WORKSPACE"));
    assert!(!input.system.contains("READ-ONLY TASK"));
    assert!(!input.system.contains("VERIFICATION TASK"));
    assert!(!input.system.contains("MUST use tools to make changes"));
    assert!(input.tools.iter().any(|tool| tool.name == "apply_patch"));
    assert_eq!(
        std::fs::read_to_string(input.context.working_dir.join("parent.txt"))?,
        "uncommitted parent work"
    );
    READY.wait().await;
    let name = &input.context.task_id;
    let patch = format!("--- a/{name}.txt\n+++ b/{name}.txt\n@@ -1 +1 @@\n-pending\n+edited\n");
    let result = input.registry.0["apply_patch"]
        .execute(serde_json::json!({"patch": patch}))
        .await?;
    assert!(result.success, "{}", result.output);
    Ok((
        "Evidence: real patch applied.\nSTATUS: completed".into(),
        1,
        1,
        AgentLoopExit::Completed,
    ))
}
