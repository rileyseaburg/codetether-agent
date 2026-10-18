//! Native services for shared step preparation.
mod prepare;
use super::super::super::Runner;
use crate::session::{Session, SessionEvent};
use tokio::sync::mpsc;
impl codetether_prompt_loop::lifecycle::start::Host for Runner<'_> {
    type Snapshot = Session;
    async fn ensure_running(&mut self) -> anyhow::Result<()> {
        crate::session::runtime::boundary::ensure_running(&self.inner.session.id).await
    }
    fn drain_steering(&mut self) {
        super::super::super::super::steering::drain_into(self.inner.session);
    }
    fn prepare(&mut self) -> anyhow::Result<()> {
        prepare::run(self)
    }
    fn inject_notices(&mut self) {
        self.inner.subagents.inject(self.inner.session);
    }
    fn events(&self) -> Option<&mpsc::Sender<SessionEvent>> {
        self.inner.events.as_ref()
    }
    fn enforce_cost(&self) -> anyhow::Result<()> {
        super::super::super::super::cost_guard::enforce_cost_budget()
    }
}
