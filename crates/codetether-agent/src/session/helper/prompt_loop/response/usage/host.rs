//! Native durable goal accounting and event bindings.
use super::super::super::Runner;
use crate::session::{Session, SessionEvent};
use codetether_prompt_loop::LoopState;
use std::time::Duration;
use tokio::sync::mpsc;
impl codetether_prompt_loop::usage::Host for Runner<'_> {
    type Snapshot = Session;
    fn target(&self) -> (&str, &str) {
        (&self.inner.model.provider_name, &self.inner.model.model_id)
    }
    fn progress(&mut self) -> &mut LoopState {
        &mut self.inner.progress
    }
    async fn account(&mut self, tokens: usize, elapsed: Duration) -> anyhow::Result<()> {
        crate::session::tasks::runtime::record_usage(&self.inner.session.id, tokens, elapsed).await
    }
    fn events(&self) -> Option<&mpsc::Sender<SessionEvent>> {
        self.inner.events.as_ref()
    }
}
