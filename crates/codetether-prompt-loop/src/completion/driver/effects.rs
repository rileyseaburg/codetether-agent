use super::super::Attempt;
use crate::LoopState;
use codetether_context_policy::relevance::Bucket;
use codetether_provider_core::CompletionResponse;
use std::{future::Future, time::Duration};
/// Accounting and recovery effects of a completion attempt.
/// # Implementors
/// Native sessions own their delegation, usage and checkpoint persistence.
pub trait Effects: Send {
    /// Record a successful call for its original delegation bucket.
    fn succeeded(&mut self, bucket: Bucket);
    /// Record usage and emit telemetry after success.
    fn usage(
        &mut self,
        response: &CompletionResponse,
        elapsed: Duration,
    ) -> impl Future<Output = ()> + Send;
    /// Access goal-failure counters to clear after successful accounting.
    fn progress(&mut self) -> &mut LoopState;
    /// Persist stream checkpoints before any retry decision.
    /// # Errors
    /// Propagates checkpoint persistence failures.
    fn checkpoint(
        &mut self,
        error: &anyhow::Error,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
    /// Apply completion recovery and update attempt context.
    /// # Errors
    /// Propagates context derivation or failover failures.
    fn recover(
        &mut self,
        step: usize,
        attempt: &mut Attempt,
        error: &anyhow::Error,
    ) -> impl Future<Output = anyhow::Result<bool>> + Send;
}
