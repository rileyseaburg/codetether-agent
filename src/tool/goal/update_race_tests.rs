//! A late PASS must not finish a replacement, edited, paused, or held goal.
use super::{args, run_with, seed};
use crate::session::tasks::{GoalStatus, runtime};
#[path = "update_race_verifier.rs"]
mod verifier;

#[tokio::test]
async fn delayed_pass_is_rejected_when_goal_identity_or_revision_changes() {
    let _lock = crate::approval::test_env::lock_env();
    let temp = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", temp.path()) };
    for case in ["replace", "edit", "pause", "hold"] {
        seed(case).await;
        let original = runtime::current(case).await.unwrap().1.goal.unwrap();
        let result = run_with(args(case, "complete"), &verifier::Mutating(case, case))
            .await
            .unwrap();
        assert!(!result.success, "{case}");
        assert!(
            result.output.contains("GOAL_VERIFICATION_STALE"),
            "{case}: {}",
            result.output
        );
        let state = runtime::current(case).await.unwrap().1;
        let goal = state.goal.unwrap();
        assert_ne!(goal.status, GoalStatus::Complete);
        if case == "replace" {
            assert_ne!(goal.id, original.id);
        }
        if case == "edit" {
            assert_eq!(goal.objective, "Different acceptance scope");
        }
        if case == "hold" {
            assert!(state.answer_review.is_some());
        }
    }
    unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
}
