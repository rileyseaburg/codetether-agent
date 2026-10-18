//! Native goal storage, events and continuation bindings.
use super::super::Runner;
use crate::session::{GoalStatus, Session, SessionEvent};
use codetether_prompt_loop::{LoopState, StepFlow};
use tokio::sync::mpsc;
impl codetether_prompt_loop::goal_recovery::Host for Runner<'_> {
    type Snapshot = Session;
    fn progress(&mut self) -> &mut LoopState {
        &mut self.inner.progress
    }
    async fn active(&mut self) -> bool {
        crate::session::tasks::runtime::current(&self.inner.session.id)
            .await
            .ok()
            .and_then(|(_, state)| state.goal)
            .is_some_and(|goal| goal.status.is_active())
    }
    fn events(&self) -> Option<&mpsc::Sender<SessionEvent>> {
        self.inner.events.as_ref()
    }
    async fn set_status(&mut self, status: GoalStatus) -> anyhow::Result<bool> {
        crate::session::tasks::runtime::set_status(&self.inner.session.id, status).await
    }
    async fn continue_goal(&mut self) -> anyhow::Result<StepFlow> {
        super::super::response::continue_goal(self).await
    }
}
