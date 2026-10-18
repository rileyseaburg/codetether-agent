//! Native worker control, completion, continuation and transcript persistence.
use super::{Native, Responses, Session};
use crate::completion::accounted as session_complete;
use anyhow::Result;
use async_trait::async_trait;
use codetether_a2a_worker_core::session_protocol::{
    driver::{Continuation, ScopedHost, SessionLoopHost},
    transcript,
};
use codetether_native_session_runtime::{boundary, continuation};

#[async_trait]
impl<S: Session, R: Sync, P: Responses<S, R>> SessionLoopHost for Native<'_, '_, S, R, P> {
    async fn before_step(&mut self, step: usize, max_steps: usize) -> Result<()> {
        boundary::ensure_running(self.session.session_id()).await?;
        codetether_session_steering::drain_into(self.session);
        transcript::maybe_inject(self.session, step, max_steps);
        Ok(())
    }
    async fn process_step(&mut self, output: &mut String) -> Result<bool> {
        let response = session_complete::complete(self.context, self.session).await?;
        P::process(&self.context.response, self.session, output, response).await
    }
    async fn continuation(&mut self) -> Result<Continuation> {
        Ok(match continuation::after_message(self.session).await? {
            continuation::Continuation::Finish => Continuation::Finish,
            continuation::Continuation::Goal => Continuation::Goal,
            continuation::Continuation::Steering => Continuation::Steering,
        })
    }
    async fn record_halt(&mut self, reason: &str) {
        transcript::record_loop_halt(self.session, reason).await;
    }
    async fn save(&mut self) -> Result<()> {
        transcript::Host::save(self.session).await
    }
}
impl<S: Session, R: Sync, P: Responses<S, R>> ScopedHost for Native<'_, '_, S, R, P> {
    fn session_id(&self) -> &str {
        self.session.session_id()
    }
    fn gate(session_id: &str) -> codetether_session_control::GateFuture<'_> {
        Box::pin(boundary::ensure_running(session_id))
    }
}
