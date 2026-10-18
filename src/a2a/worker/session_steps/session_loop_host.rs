//! Session control and persistence operations for the shared worker driver.
use super::super::{session_complete, session_failure, session_response, session_wrap_up};
use super::ApplicationHost;
use crate::session::runtime::{boundary, continuation, steering};
use anyhow::Result;
use async_trait::async_trait;
use codetether_a2a_worker_core::session_protocol::driver::{Continuation, SessionLoopHost};
#[async_trait]
impl SessionLoopHost for ApplicationHost<'_, '_> {
    async fn before_step(&mut self, step: usize, max_steps: usize) -> Result<()> {
        boundary::ensure_running(&self.session.id).await?;
        steering::drain_into(self.session);
        session_wrap_up::maybe_inject(self.session, step, max_steps);
        Ok(())
    }
    async fn process_step(&mut self, output: &mut String) -> Result<bool> {
        let response = session_complete::complete(self.context, self.session).await?;
        session_response::process_response(&self.context.response, self.session, output, response)
            .await
    }
    async fn continuation(&mut self) -> Result<Continuation> {
        Ok(match continuation::after_message(self.session).await? {
            continuation::Continuation::Finish => Continuation::Finish,
            continuation::Continuation::Goal => Continuation::Goal,
            continuation::Continuation::Steering => Continuation::Steering,
        })
    }
    async fn record_halt(&mut self, reason: &str) {
        session_failure::record_loop_halt(self.session, reason).await;
    }
    async fn save(&mut self) -> Result<()> {
        self.session.save().await
    }
}

impl codetether_a2a_worker_core::session_protocol::driver::ScopedHost for ApplicationHost<'_, '_> {
    fn session_id(&self) -> &str {
        &self.session.id
    }
    fn gate(session_id: &str) -> codetether_session_control::GateFuture<'_> {
        Box::pin(crate::session::runtime::boundary::ensure_running(
            session_id,
        ))
    }
}
