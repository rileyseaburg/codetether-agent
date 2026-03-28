use super::Agent;
use crate::session::Session;
use crate::swarm::{Actor, ActorStatus, Handler, SwarmMessage};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
impl Actor for Agent {
    fn actor_id(&self) -> &str {
        &self.info.name
    }

    fn actor_status(&self) -> ActorStatus {
        ActorStatus::Ready
    }

    async fn initialize(&mut self) -> Result<()> {
        tracing::info!(
            agent = %self.info.name,
            "Agent initialized for swarm participation"
        );
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<()> {
        tracing::info!(agent = %self.info.name, "Agent shutting down");
        Ok(())
    }
}

#[async_trait]
impl Handler<SwarmMessage> for Agent {
    type Response = SwarmMessage;

    async fn handle(&mut self, message: SwarmMessage) -> Result<Self::Response> {
        match message {
            SwarmMessage::ExecuteTask {
                task_id,
                instruction,
            } => self.handle_task_execution(task_id, instruction).await,
            SwarmMessage::ToolRequest { tool_id, arguments } => {
                self.handle_tool_request(tool_id, arguments).await
            }
            SwarmMessage::Progress { .. }
            | SwarmMessage::TaskCompleted { .. }
            | SwarmMessage::TaskFailed { .. }
            | SwarmMessage::ToolResponse { .. } => Ok(unsupported_message_response()),
        }
    }
}

impl Agent {
    async fn handle_task_execution(
        &mut self,
        task_id: String,
        instruction: String,
    ) -> Result<SwarmMessage> {
        let mut session = Session::new().await?;

        Ok(match self.execute(&mut session, &instruction).await {
            Ok(response) => SwarmMessage::TaskCompleted {
                task_id,
                result: response.text,
            },
            Err(error) => SwarmMessage::TaskFailed {
                task_id,
                error: error.to_string(),
            },
        })
    }

    async fn handle_tool_request(
        &self,
        tool_id: String,
        arguments: serde_json::Value,
    ) -> Result<SwarmMessage> {
        let result = self.execute_tool_value(&tool_id, arguments).await;
        Ok(SwarmMessage::ToolResponse { tool_id, result })
    }
}

fn unsupported_message_response() -> SwarmMessage {
    SwarmMessage::TaskFailed {
        task_id: "unknown".to_string(),
        error: "Unsupported message type".to_string(),
    }
}
