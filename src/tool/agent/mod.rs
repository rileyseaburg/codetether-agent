//! Agent Tool — spawn and communicate with sub-agents.

mod event_loop;
mod handlers;
mod helpers;
mod message;
mod policy;
mod spawn;
mod store;

use helpers::Params;

use crate::session::Session;
use crate::tool::{Tool, ToolResult};
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde_json::{Value, json};

#[derive(Clone)]
pub struct AgentSnapshot {
    pub name: String,
    pub instructions: String,
    pub session: Session,
}

pub fn list_agent_snapshots() -> Vec<AgentSnapshot> { store::snapshots() }
pub fn remove_agent(name: &str) -> bool { store::remove(name).is_some() }

pub struct AgentTool;
impl AgentTool { pub fn new() -> Self { Self } }
impl Default for AgentTool { fn default() -> Self { Self::new() } }

#[async_trait]
impl Tool for AgentTool {
    fn id(&self) -> &str { "agent" }
    fn name(&self) -> &str { "Sub-Agent" }

    fn description(&self) -> &str {
        "Spawn and communicate with specialized sub-agents. Actions: spawn, message, list, kill. \
         Spawned agents must use a free/subscription-eligible model."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": { "type": "string", "enum": ["spawn", "message", "list", "kill"] },
                "name": { "type": "string", "description": "Agent name" },
                "instructions": { "type": "string", "description": "System instructions (spawn)" },
                "message": { "type": "string", "description": "Message to send" },
                "model": { "type": "string", "description": "Model (spawn). Must be free/subscription-eligible." }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;
        match p.action.as_str() {
            "spawn" => spawn::handle_spawn(&p).await,
            "message" => message::handle_message(&p).await,
            "list" => Ok(handlers::handle_list()),
            "kill" => {
                let name = p.name.as_ref().context("name required for kill")?;
                Ok(handlers::handle_kill(name))
            }
            _ => Ok(ToolResult::error(format!(
                "Unknown action: {}. Valid: spawn, message, list, kill", p.action
            ))),
        }
    }
}
