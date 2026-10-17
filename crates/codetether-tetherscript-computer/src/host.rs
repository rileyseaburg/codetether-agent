//! Application policy and backend operations used by the computer capability.
use async_trait::async_trait;
use codetether_tool_core::ToolResult;
use serde::de::DeserializeOwned;
use serde_json::Value;
/// Binds the authority to the existing computer-use backend and policy.
///
/// # Implementors
/// Use the same payload type and policy evaluator as direct computer-use calls.
#[async_trait]
pub trait Host: Clone + std::fmt::Debug + Send + Sync + 'static {
    /// Deserializable input accepted by the platform backend.
    type Input: DeserializeOwned + Send + Sync;
    /// Execute an input after capability and policy checks have succeeded.
    /// # Errors
    /// Returns backend errors unchanged for the capability to annotate.
    async fn dispatch(input: &Self::Input) -> anyhow::Result<ToolResult>;
    /// Return a blocking result or allow the computer-use invocation.
    async fn evaluate_tool_invocation(tool: &str, args: &Value) -> Option<ToolResult>;
}
