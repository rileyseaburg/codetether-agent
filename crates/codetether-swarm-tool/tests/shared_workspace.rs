//! Mocked scheduling with twenty real, workspace-scoped patch executions.
#[path = "shared_support/mod.rs"]
mod support;
use codetether_swarm_tool::runtime;
use serde_json::json;

#[tokio::test]
async fn twenty_workers_edit_one_shared_workspace() {
    let dir = tempfile::tempdir_in(std::env::current_dir().unwrap()).unwrap();
    std::fs::write(dir.path().join("parent.txt"), "uncommitted parent work").unwrap();
    let tasks: Vec<_> = (0..20)
        .map(|i| {
            std::fs::write(dir.path().join(format!("worker-{i}.txt")), "pending").unwrap();
            json!({"id": format!("worker-{i}"), "name": "Review",
            "instruction": "Inspect the assigned file", "needs_worktree": i % 2 == 0})
        })
        .collect();
    let response = tokio::time::timeout(
        std::time::Duration::from_secs(30),
        runtime::execute::<support::TestHost>(json!({
            "tasks": tasks, "concurrency_limit": 20, "aggregation_strategy": "all",
            "__ct_parent_workspace": dir.path(),
        })),
    )
    .await
    .unwrap()
    .unwrap();
    assert!(response.success, "{}", response.output);
    let result: serde_json::Value = serde_json::from_str(&response.output).unwrap();
    assert_eq!(result["summary"]["success"], 20);
    for i in 0..20 {
        assert_eq!(
            std::fs::read_to_string(dir.path().join(format!("worker-{i}.txt"))).unwrap(),
            "edited"
        );
    }
    assert_eq!(
        std::fs::read_to_string(dir.path().join("parent.txt")).unwrap(),
        "uncommitted parent work"
    );
    assert!(!dir.path().join(".codetether-worktrees").exists());
}
