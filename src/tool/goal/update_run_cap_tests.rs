//! Rejection cap pauses the goal and verdicts are persisted.

use super::run_with;
use crate::approval::test_env::lock_env;
use crate::session::tasks::GoalStatus;

#[path = "update_run_fixtures.rs"]
mod fixtures;
use fixtures::{Scripted, args, seed, status};

#[tokio::test]
async fn rejections_are_logged_and_cap_pauses_goal() {
    let _lock = lock_env();
    let temp = tempfile::tempdir().unwrap();
    unsafe {
        std::env::set_var("CODETETHER_DATA_DIR", temp.path());
        std::env::set_var(crate::tool::goal::attempt_cap::MAX_ATTEMPTS_ENV, "2");
    }
    seed("verify-cap").await;
    let verifier = Scripted("FAIL — tests — not run\nVERDICT: FAIL");
    for _ in 0..2 {
        let result = run_with(args("verify-cap", "complete"), &verifier)
            .await
            .unwrap();
        assert!(result.output.contains("not run"));
    }
    let capped = run_with(args("verify-cap", "complete"), &verifier)
        .await
        .unwrap();
    assert!(!capped.success && capped.output.contains("paused"));
    assert_eq!(status("verify-cap").await, GoalStatus::Paused);
    let bypass = run_with(args("verify-cap", "complete"), &Scripted("VERDICT: PASS"))
        .await
        .unwrap();
    assert!(!bypass.success);
    assert_eq!(status("verify-cap").await, GoalStatus::Paused);
    let log = crate::tool::goal::verdict_log::read("verify-cap")
        .await
        .unwrap();
    assert_eq!(log.len(), 3);
    assert!(log.iter().all(|r| !r.passed));
    assert!(log[2].escalated);
    unsafe {
        std::env::remove_var(crate::tool::goal::attempt_cap::MAX_ATTEMPTS_ENV);
        std::env::remove_var("CODETETHER_DATA_DIR");
    }
}
