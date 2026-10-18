//! Swarm message handling for agents.
//!
//! This module maps swarm task and tool messages onto the normal agent
//! execution and tool-dispatch paths.
//!
//! # Examples
//!
//! ```ignore
//! let reply = agent.handle(message).await?;
//! ```

use super::task_execution::unsupported_message_response;
use crate::{Agent, Host};
use anyhow::Result;
use async_trait::async_trait;
use codetether_swarm_core::{Handler, SwarmMessage};

#[async_trait]
impl<H: Host> Handler<SwarmMessage> for Agent<H> {
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
