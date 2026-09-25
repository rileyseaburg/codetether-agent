//! `update_goal` only changes state when the independent verifier passes.

use super::run_with;
use crate::approval::test_env::lock_env;
use crate::session::tasks::GoalStatus;

#[path = "update_run_fixtures.rs"]
mod fixtures;
use fixtures::{Scripted, args, seed, status};

#[tokio::test]
async fn rejected_claim_keeps_goal_active_and_returns_findings() {
    let _lock = lock_env();
    let temp = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", temp.path()) };
    seed("verify-reject").await;
    let verifier = Scripted("FAIL — docs — missing README section\nVERDICT: FAIL");
    let result = run_with(args("verify-reject", "complete"), &verifier)
        .await
        .unwrap();
    assert!(!result.success);
    assert!(result.output.contains("missing README section"));
    assert_eq!(status("verify-reject").await, GoalStatus::Active);
    unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
}

#[tokio::test]
async fn passed_claim_applies_transition() {
    let _lock = lock_env();
    let temp = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", temp.path()) };
    seed("verify-pass").await;
    let verifier = Scripted("PASS — export — src/export.rs\nVERDICT: PASS");
    let result = run_with(args("verify-pass", "complete"), &verifier)
        .await
        .unwrap();
    assert!(result.success, "{}", result.output);
    assert_eq!(status("verify-pass").await, GoalStatus::Complete);
    unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
}
