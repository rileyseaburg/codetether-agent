use crate::Call;
use codetether_tool_outcome::Outcome;
use std::future::Future;
/// Native approval-aware tool execution and outcome rendering.
/// # Implementors
/// The runner retains its approval, persistence and tool-dispatch services.
pub trait Execute {
    /// Execute a permitted normalized call.
    /// # Errors
    /// Propagates execution-stage failures.
    fn execute(&mut self, call: &Call) -> impl Future<Output = anyhow::Result<Outcome>> + Send;
}
