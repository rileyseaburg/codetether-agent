//! Adapt application state and message execution to shared HTTP routes.
use super::*;
#[async_trait::async_trait]
impl codetether_a2a_http::server::ServerHost for A2AServer {
    fn agent_card(&self) -> &AgentCard {
        &self.agent_card
    }
    fn tasks(&self) -> &DashMap<String, Task> {
        &self.tasks
    }
    async fn send_message(
        &self,
        request: JsonRpcRequest,
    ) -> Result<serde_json::Value, JsonRpcError> {
        codetether_a2a_http::admission::send(self, request).await
    }
    async fn stream_message(
        &self,
        request: JsonRpcRequest,
    ) -> Result<serde_json::Value, JsonRpcError> {
        handle_message_stream(self, request).await
    }
}
