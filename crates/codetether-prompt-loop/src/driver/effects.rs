use super::ParsedCall;
use crate::StepFlow;
use codetether_provider_core::CompletionResponse;
use std::future::Future;
/// Native session-control, output and response-repair effects.
/// # Implementors
/// Session runners supply these asynchronous effects in their established order.
pub trait Effects {
    /// Check the active execution boundary.
    /// # Errors
    /// Propagates cancellation or control errors.
    fn ensure_running(&mut self) -> impl Future<Output = anyhow::Result<()>> + Send;
    /// Publish and accumulate visible assistant output.
    fn emit(
        &mut self,
        step: usize,
        response: &CompletionResponse,
    ) -> impl Future<Output = ()> + Send;
    /// Apply terminal validation and goal continuation.
    /// # Errors
    /// Propagates validation or continuation failures.
    fn terminal(
        &mut self,
        response: CompletionResponse,
    ) -> impl Future<Output = anyhow::Result<StepFlow>> + Send;
    /// Repair truncated calls and report whether an immediate retry is needed.
    fn repair(
        &mut self,
        response: &CompletionResponse,
        calls: &[ParsedCall],
        truncated: &[(String, String)],
    ) -> impl Future<Output = bool> + Send;
}
