//! Application hooks for inbound message admission.
use super::*;
use async_trait::async_trait;

/// Application state and execution hooks for message admission.
///
/// # Implementors
/// Implementors expose the task table and agent card, publish bus messages,
/// and dispatch accepted prompts through their runtime.
#[async_trait]
pub trait MessageAdmissionHost: Clone + Send + Sync + 'static {
    /// Table used by task creation, validation and responses.
    fn tasks(&self) -> &DashMap<String, Task>;
    /// Agent card used to describe this peer in introduction replies.
    fn agent_card(&self) -> &AgentCard;
    /// Publish a completed outbound response to application observers.
    fn emit_outbound(&self, task_id: &str, outbound: &Message);
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
