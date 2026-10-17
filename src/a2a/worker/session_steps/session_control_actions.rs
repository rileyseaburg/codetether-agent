//! Initial provider call injects one control action through the shared runtime.

use super::{Mock, Scenario};
use crate::session::{
    runtime::{CONTROL, goals, steering},
    tasks::GoalStatus,
};

impl Mock {
    pub(super) async fn first_action(&self) -> anyhow::Result<bool> {
        match self.scenario {
            Scenario::Steering => assert!(steering::push(
                &self.id,
                steering::SteeringInput::new("use the cloned repos".into(), vec![])
            )),
            Scenario::Cancel => {
                assert!(CONTROL.cancel_session(&self.id));
                return futures::future::pending().await;
            }
            Scenario::Pause | Scenario::Hold => {
                if matches!(self.scenario, Scenario::Hold) {
                    goals::answer_review::begin(&self.id, "Why?").await?;
                } else {
                    goals::set_status(&self.id, GoalStatus::Paused).await?;
                }
                return Ok(true);
            }
            Scenario::Goal => {}
        }
        Ok(false)
    }
}
