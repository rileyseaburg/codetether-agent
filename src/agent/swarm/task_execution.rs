//! Task-specific swarm helpers for agents.
//!
//! This module holds the helper methods used by the swarm handler so the main
//! handler implementation file stays focused on dispatch logic.
//!
//! # Examples
//!
//! ```ignore
//! let reply = unsupported_message_response();
//! ```

use crate::agent::Agent;
use crate::session::Session;
use crate::swarm::SwarmMessage;
use anyhow::Result;

impl Agent {
    pub(super) async fn handle_task_execution(
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

    pub(super) async fn handle_tool_request(
        &self,
        tool_id: String,
        arguments: serde_json::Value,
    ) -> Result<SwarmMessage> {
        let result = self.execute_tool_value(&tool_id, arguments).await;
        Ok(SwarmMessage::ToolResponse { tool_id, result })
    }
}

/// Returns a failure message for swarm events the agent does not handle.
///
/// # Examples
///
/// ```ignore
/// let reply = unsupported_message_response();
/// ```
pub(super) fn unsupported_message_response() -> SwarmMessage {
    SwarmMessage::TaskFailed {
        task_id: "unknown".to_string(),
        error: "Unsupported message type".to_string(),
    }
}
