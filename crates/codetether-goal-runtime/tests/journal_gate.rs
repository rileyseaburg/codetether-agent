use codetether_goal_core::TaskLog;
use codetether_goal_runtime::{Journal, ensure_running};

struct Journals;
impl Journal for Journals {
    fn for_session(id: &str) -> anyhow::Result<TaskLog> {
        Ok(TaskLog::at(id))
    }
}

#[tokio::test]
async fn damaged_governance_stops_the_execution_gate() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("goal.jsonl");
    std::fs::write(&path, b"{broken}\n").unwrap();
    assert!(
        ensure_running::<Journals>(path.to_str().unwrap())
            .await
            .is_err()
    );
}
