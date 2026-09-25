//! A `blocked` claim is rejected unless controlled opposition confirms it.

use super::run_with;
use crate::approval::test_env::lock_env;
use crate::session::tasks::GoalStatus;

#[path = "update_run_fixtures.rs"]
mod fixtures;
use fixtures::{Scripted, args, seed, status};

#[tokio::test]
async fn opposed_blocked_claim_keeps_goal_active() {
    let _lock = lock_env();
    let temp = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", temp.path()) };
    seed("blocked-opposed").await;
    let opposition =
        Scripted("WORKAROUND — dirty tree — push from a clean worktree\nVERDICT: FAIL");
    let result = run_with(args("blocked-opposed", "blocked"), &opposition)
        .await
        .unwrap();
    assert!(!result.success);
    assert!(result.output.contains("push from a clean worktree"));
    assert_eq!(status("blocked-opposed").await, GoalStatus::Active);
    unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
}

#[tokio::test]
async fn confirmed_blocker_marks_goal_blocked() {
    let _lock = lock_env();
    let temp = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", temp.path()) };
    seed("blocked-confirmed").await;
    let opposition = Scripted("REAL — Stripe key absent from Vault\nVERDICT: PASS");
    let result = run_with(args("blocked-confirmed", "blocked"), &opposition)
        .await
        .unwrap();
    assert!(result.success, "{}", result.output);
    assert_eq!(status("blocked-confirmed").await, GoalStatus::Blocked);
    unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
}
