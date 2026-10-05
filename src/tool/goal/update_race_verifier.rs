//! Deterministically change native goal state while verification is in flight.
use crate::session::tasks::{GoalStatus, TaskEvent, TaskLog, control, runtime};
use crate::tool::goal::verify::{VerificationRequest, VerifierAgent};

pub(super) struct Mutating(pub &'static str, pub &'static str);
#[async_trait::async_trait]
impl VerifierAgent for Mutating {
    async fn review(&self, _: &VerificationRequest) -> anyhow::Result<String> {
        match self.1 {
            "replace" => {
                TaskLog::for_session(self.0)?
                    .append(&TaskEvent::GoalCleared {
                        at: chrono::Utc::now(),
                        reason: "new user goal".into(),
                    })
                    .await?;
                super::super::fixtures::seed(self.0).await;
            }
            "pause" => {
                runtime::set_status(self.0, GoalStatus::Paused).await?;
            }
            "hold" => {
                runtime::answer_review::begin(self.0, "Explain the result first").await?;
            }
            "edit" => {
                let goal = runtime::current(self.0).await?.1.goal.unwrap();
                control::update(
                    self.0,
                    serde_json::from_value(serde_json::json!({
                        "goalId": goal.id, "updatedAt": goal.last_updated_at,
                        "action": "edit", "objective": "Different acceptance scope"
                    }))?,
                )
                .await?;
            }
            _ => unreachable!("test mutation"),
        }
        Ok("VERDICT: PASS".into())
    }
}
