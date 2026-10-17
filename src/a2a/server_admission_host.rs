//! Bind shared admission to application bus, intro and execution operations.
use super::*;
#[async_trait::async_trait]
impl codetether_a2a_http::admission::MessageAdmissionHost for A2AServer {
    fn tasks(&self) -> &DashMap<String, Task> {
        &self.tasks
    }
    fn handle_intro(
        &self,
        task_id: &str,
        inbound: &Message,
        task: Task,
    ) -> Result<serde_json::Value, JsonRpcError> {
        server_intro::handle_intro(self, task_id, inbound, task)
    }
    fn emit_inbound(&self, task_id: &str, inbound: &Message) {
        emit_a2a_inbound(self, task_id, inbound);
    }
    async fn execute_send(
        self,
        task_id: String,
        params: MessageSendParams,
        prompt: String,
        blocking: bool,
    ) {
        server_message_send_turn::run(self, task_id, params, prompt, blocking).await;
    }
    fn spawn_stream(&self, params: &MessageSendParams, task_id: &str, prompt: String) {
        stream::spawn::spawn(self, params, task_id, prompt);
    }
}
