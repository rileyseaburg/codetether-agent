//! Native completion-recovery services.
use super::super::{super::Runner, context::Attempt};
use crate::{
    provider::Message,
    session::{DerivePolicy, DerivedContext, Session, SessionEvent},
};
use tokio::sync::mpsc;
pub(super) struct Native<'a, 'session> {
    pub runner: &'a mut Runner<'session>,
    pub attempt: &'a mut Attempt,
}
impl codetether_prompt_loop::completion::Host for Native<'_, '_> {
    type Snapshot = Session;
    fn attempt(&mut self) -> &mut Attempt {
        self.attempt
    }
    async fn derive(
        &mut self,
        policy: DerivePolicy,
        keep: Option<usize>,
    ) -> anyhow::Result<DerivedContext> {
        super::super::derived::derive(self.runner, policy, keep).await
    }
    fn failed(&mut self) {
        self.runner.session.metadata.delegation.update(
            &self.runner.model.provider_name,
            crate::session::delegation_skills::MODEL_CALL,
            self.attempt.bucket,
            false,
        );
    }
    fn failover(&mut self) -> anyhow::Result<bool> {
        super::super::failover::apply(self.runner, self.attempt.bucket)
    }
    async fn proactive(&mut self, step: usize) -> Option<Message> {
        super::super::derived::lsp(self.runner, step).await
    }
    fn events(&self) -> Option<&mpsc::Sender<SessionEvent>> {
        self.runner.events.as_ref()
    }
}
