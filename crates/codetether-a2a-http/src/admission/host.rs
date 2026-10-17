//! Application hooks for inbound message admission.
use super::*;
use async_trait::async_trait;

/// Application state and execution hooks for message admission.
///
/// # Implementors
/// Implementors expose the task table, publish inbound bus messages, acknowledge
/// introductions and dispatch accepted prompts through their runtime.
#[async_trait]
pub trait MessageAdmissionHost: Clone + Send + Sync + 'static {
    /// Table used by task creation, validation and responses.
    fn tasks(&self) -> &DashMap<String, Task>;
    /// Complete an introduction without executing a prompt.
    /// # Errors
    /// Returns a JSON-RPC error if the acknowledgement cannot be serialized.
    fn handle_intro(
        &self,
        task_id: &str,
        inbound: &Message,
        task: Task,
    ) -> Result<Value, JsonRpcError>;
    /// Publish the accepted inbound message to application observers.
    fn emit_inbound(&self, task_id: &str, inbound: &Message);
    /// Execute and settle an accepted message/send task.
    async fn execute_send(
        self,
        task_id: String,
        params: MessageSendParams,
        prompt: String,
        blocking: bool,
    );
    /// Schedule an accepted stream task in the background.
    fn spawn_stream(&self, params: &MessageSendParams, task_id: &str, prompt: String);
}
