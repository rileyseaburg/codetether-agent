//! Cross-process proof that an approval receipt has one winner.

#[path = "store_claim_process_child.rs"]
mod child;

use super::{ApprovalStore, test_env::ScopedEnv, test_env::lock_env};
use std::process::Command;

const RESOURCE: &str = "bash:process-race";

#[test]
fn independent_processes_claim_exactly_once() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let _env = ScopedEnv::data_dir_with_access(data.path(), crate::config::AccessMode::Ask);
    let store = ApprovalStore::open_default().expect("store");
    let request = store
        .create_request("bash", "execute", RESOURCE, "race")
        .expect("request");
    store
        .approve(&request.id, "test", "allow")
        .expect("approve");
    let marker = data.path().join("claimed.txt");
    std::fs::write(&marker, "").expect("marker");
    let executable = std::env::current_exe().expect("test executable");
    let mut children = Vec::new();
    for _ in 0..8 {
        children.push(
            Command::new(&executable)
                .args([
                    "--exact",
                    "approval::store_claim_process_tests::child::claim_child",
                ])
                .env("CODETETHER_CLAIM_CHILD", &request.id)
                .env("CODETETHER_CLAIM_MARKER", &marker)
                .spawn()
                .expect("spawn child"),
        );
    }
    for child in &mut children {
        assert!(child.wait().expect("wait").success());
    }
    let claims = std::fs::read_to_string(marker).expect("claims");
    assert_eq!(claims.lines().count(), 1);
}
