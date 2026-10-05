//! Mocked verifier authentication failures are not substantive rejections.
use super::{Scripted, args, run_with, seed, status};
use crate::session::tasks::GoalStatus;
use crate::tool::goal::{
    attempt_cap, verdict_log,
    verify::{VerificationRequest, VerifierAgent},
};

struct Expired;
#[async_trait::async_trait]
impl VerifierAgent for Expired {
    async fn review(&self, _: &VerificationRequest) -> anyhow::Result<String> {
        anyhow::bail!("expired Bedrock bearer token");
    }
}

#[tokio::test]
async fn expired_credentials_never_consume_rejections_or_pause_the_goal() {
    let _lock = crate::approval::test_env::lock_env();
    let temp = tempfile::tempdir().unwrap();
    unsafe { std::env::set_var("CODETETHER_DATA_DIR", temp.path()) };
    let session = "expired-verifier";
    seed(session).await;
    for claimed in ["complete", "blocked"] {
        for _ in 0..7 {
            let result = run_with(args(session, claimed), &Expired).await.unwrap();
            assert!(!result.success);
            assert!(result.output.contains("GOAL_VERIFIER_UNAVAILABLE"));
            assert!(result.output.contains("expired Bedrock bearer token"));
            assert_eq!(status(session).await, GoalStatus::Active);
        }
    }
    let log = verdict_log::read(session).await.unwrap();
    assert_eq!(log.len(), 14);
    assert!(
        log.iter()
            .all(|record| record.unavailable && !record.escalated)
    );
    assert_eq!(
        attempt_cap::rejections_since_reset(&log, &log[0].goal_id),
        0
    );
    let result = run_with(args(session, "complete"), &Scripted("VERDICT: PASS"))
        .await
        .unwrap();
    assert!(result.success);
    assert_eq!(status(session).await, GoalStatus::Complete);
    unsafe { std::env::remove_var("CODETETHER_DATA_DIR") };
}
