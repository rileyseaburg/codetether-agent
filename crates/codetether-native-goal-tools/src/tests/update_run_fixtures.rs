//! Fixtures for `update_goal` verifier-gate tests.

use async_trait::async_trait;
use codetether_goal_tools::UpdateArgs as Args;
use codetether_goal_verification::{VerificationRequest, VerifierAgent};
use codetether_native_goals::GoalStatus;

/// Verifier that returns a fixed report.
pub(super) struct Scripted(pub &'static str);

#[async_trait]
impl VerifierAgent for Scripted {
    async fn review(&self, _: &VerificationRequest) -> anyhow::Result<String> {
        Ok(self.0.to_string())
    }
}

/// `update_goal` arguments for `session` requesting `status`.
pub(super) fn args(session: &str, status: &str) -> Args {
    Args {
        status: status.into(),
        evidence: "cargo test passed".into(),
        session_id: Some(session.into()),
        current_model: None,
        workspace: None,
    }
}

/// Create an active goal for `session` through the real `create_goal` path.
pub(super) async fn seed(session: &str) {
    let create = codetether_goal_tools::CreateArgs {
        objective: "Ship export".into(),
        session_id: Some(session.into()),
    };
    assert!(
        codetether_goal_tools::create::<crate::tests::TestHost>(create)
            .await
            .unwrap()
            .success
    );
}

/// Persisted status of the goal for `session`.
pub(super) async fn status(session: &str) -> GoalStatus {
    let (_, state) = codetether_native_goals::runtime::current(session)
        .await
        .unwrap();
    state.goal.unwrap().status
}
