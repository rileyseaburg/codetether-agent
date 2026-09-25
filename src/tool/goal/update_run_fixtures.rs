//! Fixtures for `update_goal` verifier-gate tests.

use crate::session::tasks::GoalStatus;
use crate::tool::goal::update::Args;
use crate::tool::goal::verify::{VerificationRequest, VerifierAgent};
use async_trait::async_trait;

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
    let create = crate::tool::goal::create::Args {
        objective: "Ship export".into(),
        token_budget: None,
        session_id: Some(session.into()),
    };
    assert!(
        crate::tool::goal::create_run::run(create)
            .await
            .unwrap()
            .success
    );
}

/// Persisted status of the goal for `session`.
pub(super) async fn status(session: &str) -> GoalStatus {
    let (_, state) = crate::session::tasks::runtime::current(session)
        .await
        .unwrap();
    state.goal.unwrap().status
}
