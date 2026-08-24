//! Reusable session grants execute through the real write backend.

use crate::approval::{ApprovalStore, session_grants, test_env::{ScopedEnv, lock_env}};
use crate::config::AccessMode;
use crate::tool::{Tool, file::WriteTool};
use serde_json::json;

struct Reset;
impl Drop for Reset {
    fn drop(&mut self) { session_grants::reset(); }
}

#[tokio::test]
async fn session_grant_reaches_backend_but_cannot_cross_sessions() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    session_grants::reset();
    let _reset = Reset;
    let target = std::env::current_dir().unwrap().join("target").join(
        format!("session-grant-write-{}.txt", uuid::Uuid::new_v4()),
    );
    let args = json!({
        "path":target, "content":"session approved", "__ct_session_id":"session-a"
    });
    let tool = WriteTool::new();
    let blocked = tool.execute(args.clone()).await.unwrap();
    let id = blocked.metadata["approval_request_id"].as_str().unwrap();
    let receipt = ApprovalStore::open_default().unwrap().approve(id, "test", "session").unwrap();
    session_grants::grant(&receipt);

    assert!(tool.execute(args.clone()).await.unwrap().success);
    assert!(tool.execute(args.clone()).await.unwrap().success);
    let mut other = args;
    other["__ct_session_id"] = json!("session-b");
    assert!(!tool.execute(other).await.unwrap().success);
    assert_eq!(std::fs::read_to_string(&target).unwrap(), "session approved");
    std::fs::remove_file(target).unwrap();
}