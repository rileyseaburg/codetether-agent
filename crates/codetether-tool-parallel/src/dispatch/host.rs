use super::ToolCall;
use serde_json::Value;
use std::future::Future;
/// Execution services used by the ordered batch dispatcher.
/// # Implementors
/// Native prompt runners normalize single calls and execute parallel groups.
pub trait Host: Send {
    /// Whether this caller can execute parallel read groups.
    fn allow_parallel(&self) -> bool;
    /// Try a parallel group; false requests sequential fallback.
    fn parallel(&mut self, calls: &[ToolCall]) -> impl Future<Output = bool> + Send;
    /// Normalize and execute one call; true stops the remaining batch.
    /// # Errors
    /// Propagates native call execution errors.
    fn single(
        &mut self,
        step: usize,
        id: &str,
        name: &str,
        input: &Value,
    ) -> impl Future<Output = anyhow::Result<bool>> + Send;
}
