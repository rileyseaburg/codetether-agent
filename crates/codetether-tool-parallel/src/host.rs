//! Session services required by shared batch preparation and recording.
use crate::Output;
use async_trait::async_trait;
use codetether_provider_core::Message;
use serde_json::Value;
/// Supplies session state, persistence, input context and output routing.
/// # Implementors
/// Applications retain their own session storage and RLM routing policy.
#[async_trait]
pub trait Host: Sync {
    /// Stable session ID used for execution audit correlation.
    fn id(&self) -> &str;
    /// Add the same runtime context used for sequential tool execution.
    fn enrich(&self, input: &Value) -> Value;
    /// Persist the session before tools execute.
    /// # Errors
    /// Propagates storage failures; a failed save prevents batch execution.
    async fn save(&self) -> anyhow::Result<()>;
    /// Route output through the session's compaction and feedback policies.
    fn route(&self, output: &Output) -> String;
    /// Append a tool-role message with all returned attachments.
    fn record(&mut self, message: Message);
}
