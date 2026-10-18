use crate::Call;
use codetether_tool_outcome::Outcome;
use std::future::Future;
/// Native effects surrounding one tool execution.
/// # Implementors
/// The prompt runner supplies stream, transcript, discovery and publication hooks.
pub trait Effects {
    /// Emit the initial tool event.
    fn start(&mut self, call: &Call) -> impl Future<Output = ()> + Send;
    /// Handle the virtual list-tools call.
    /// # Errors
    /// Propagates discovery-stage failures.
    fn discover(&mut self, call: &Call) -> impl Future<Output = anyhow::Result<bool>> + Send;
    /// Publish the agent-bus request before policy checks.
    fn request(&mut self, step: usize, call: &Call);
    /// Record an immediate policy rejection.
    fn rejected(&mut self, call: &Call, content: String) -> impl Future<Output = ()> + Send;
    /// Reconcile session-editing tools before publishing the result.
    /// # Errors
    /// Propagates refresh failures.
    fn refresh(
        &mut self,
        call: &Call,
        success: bool,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
    /// Publish and persist the completed outcome.
    /// # Errors
    /// Propagates publication or persistence failures.
    fn publish(
        &mut self,
        step: usize,
        call: &Call,
        outcome: Outcome,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}
