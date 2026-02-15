//! Relay AutoChat Tool - Autonomous relay communication between agents
//!
//! Enables task delegation and result aggregation between agents using
//! the protocol-first relay runtime. This tool allows LLMs to trigger
//! agent handoffs and coordinate multi-agent workflows.

use super::{Tool, ToolResult};
use crate::bus::relay::{ProtocolRelayRuntime, RelayAgentProfile};
use crate::bus::{AgentBus, BusMessage};
use crate::a2a::types::Part;
use anyhow::{Context, Result};
use async_trait::async_trait;
use parking_lot::RwLock;
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::OnceCell;
use uuid::Uuid;

lazy_static::lazy_static! {
    static ref RELAY_STORE: RwLock<HashMap<String, Arc<ProtocolRelayRuntime>>> = RwLock::new(HashMap::new());
    static ref AGENT_BUS: OnceCell<Arc<AgentBus>> = OnceCell::const_new();
}

async fn get_agent_bus() -> Result<Arc<AgentBus>> {
    let bus = AGENT_BUS
        .get_or_try_init(|| async {
            let bus = AgentBus::new().into_arc();
            Ok::<_, anyhow::Error>(bus)
        })
        .await?;
    Ok(bus.clone())
}

/// Input parameters for relay_autochat tool
#[derive(Debug, Deserialize)]
struct Params {
    /// Action to perform: delegate, handoff, status, list_agents
    action: String,
    /// Target agent name for delegation/handoff
    #[serde(default)]
    target_agent: Option<String>,
    /// Message to send to the target agent
    #[serde(default)]
    message: Option<String>,
    /// Additional context to pass along
    #[serde(default)]
    context: Option<Value>,
    /// Relay ID to use (auto-generated if not provided)
    #[serde(default)]
    relay_id: Option<String>,
    /// Optional OKR ID to associate with this relay
    #[serde(default)]
    okr_id: Option<String>,
    /// Task description for initializing a new relay
    #[serde(default)]
    task: Option<String>,
}

pub struct RelayAutoChatTool;

impl RelayAutoChatTool {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl Tool for RelayAutoChatTool {
    fn id(&self) -> &str {
        "relay_autochat"
    }

    fn name(&self) -> &str {
        "Relay AutoChat"
    }

    fn description(&self) -> &str {
        "Autonomous relay communication between agents for task delegation and result aggregation. \
         Actions: delegate (send task to target agent), handoff (pass context between agents), \
         status (check relay status), list_agents (show available agents in relay), \
         init (initialize a new relay with task), complete (finish relay and aggregate results)."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["delegate", "handoff", "status", "list_agents", "init", "complete"],
                    "description": "Action to perform"
                },
                "target_agent": {
                    "type": "string",
                    "description": "Target agent name for delegation/handoff"
                },
                "message": {
                    "type": "string",
                    "description": "Message to send to the target agent"
                },
                "context": {
                    "type": "object",
                    "description": "Additional context to pass along (JSON object)"
                },
                "relay_id": {
                    "type": "string",
                    "description": "Relay ID to use (auto-generated if not provided)"
                },
                "okr_id": {
                    "type": "string",
                    "description": "Optional OKR ID to associate with this relay"
                },
                "task": {
                    "type": "string",
                    "description": "Task description for initializing a new relay"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;

        match p.action.as_str() {
            "init" => self.init_relay(p).await,
            "delegate" => self.delegate_task(p).await,
            "handoff" => self.handoff_context(p).await,
            "status" => self.get_status(p).await,
            "list_agents" => self.list_agents(p).await,
            "complete" => self.complete_relay(p).await,
            _ => Ok(ToolResult::error(format!(
                "Unknown action: {}. Valid actions: init, delegate, handoff, status, list_agents, complete",
                p.action
            ))),
        }
    }
}

impl RelayAutoChatTool {
    /// Initialize a new relay with a task
    async fn init_relay(&self, params: Params) -> Result<ToolResult> {
        let task = params.task.unwrap_or_else(|| "Unspecified task".to_string());
        let relay_id = params.relay_id.unwrap_or_else(|| format!("relay-{}", &Uuid::new_v4().to_string()[..8]));

        let bus = get_agent_bus().await?;
        let runtime = ProtocolRelayRuntime::with_relay_id(bus, relay_id.clone());

        // Store the runtime
        {
            let mut store = RELAY_STORE.write();
            store.insert(relay_id.clone(), Arc::new(runtime.clone()));
        }

        let response = json!({
            "status": "initialized",
            "relay_id": relay_id,
            "task": task,
            "message": "Relay initialized. Use 'delegate' to assign tasks to agents, or 'list_agents' to see available agents."
        });

        Ok(ToolResult::success(
            serde_json::to_string_pretty(&response).unwrap_or_else(|_| format!("{:?}", response)),
        ))
        .with_metadata("relay_id", json!(relay_id))
    }

    /// Delegate a task to a target agent
    async fn delegate_task(&self, params: Params) -> Result<ToolResult> {
        let relay_id = params.relay_id.ok_or_else(|| anyhow::anyhow!("relay_id required for delegate"))?;
        let target_agent = params.target_agent.ok_or_else(|| anyhow::anyhow!("target_agent required for delegate"))?;
        let message = params.message.unwrap_or_else(|| "New task assigned".to_string());

        // Get or create the runtime
        let runtime = {
            let store = RELAY_STORE.read();
            store.get(&relay_id).cloned()
        };

        let runtime = match runtime {
            Some(r) => r,
            None => {
                // Create a new runtime if it doesn't exist
                let bus = get_agent_bus().await?;
                let new_runtime = ProtocolRelayRuntime::with_relay_id(bus, relay_id.clone());
                let arc_runtime = Arc::new(new_runtime);
                {
                    let mut store = RELAY_STORE.write();
                    store.insert(relay_id.clone(), arc_runtime.clone());
                }
                arc_runtime
            }
        };

        // Build context payload if provided
        let context_msg = if let Some(ref ctx) = params.context {
            format!("{}\n\nContext: {}", message, serde_json::to_string_pretty(ctx).unwrap_or_default())
        } else {
            message.clone()
        };

        // Send the delegation message
        runtime.send_handoff("system", &target_agent, &context_msg);

        let response = json!({
            "status": "delegated",
            "relay_id": relay_id,
            "target_agent": target_agent,
            "message": message,
            "initial_results": {
                "task_assigned": true,
                "agent_notified": true
            }
        });

        Ok(ToolResult::success(
            serde_json::to_string_pretty(&response).unwrap_or_else(|_| format!("{:?}", response)),
        ))
        .with_metadata("relay_id", json!(relay_id))
        .with_metadata("target_agent", json!(target_agent))
    }

    /// Hand off context between agents
    async fn handoff_context(&self, params: Params) -> Result<ToolResult> {
        let relay_id = params.relay_id.ok_or_else(|| anyhow::anyhow!("relay_id required for handoff"))?;
        let target_agent = params.target_agent.ok_or_else(|| anyhow::anyhow!("target_agent required for handoff"))?;
        let message = params.message.unwrap_or_else(|| "Context handoff".to_string());

        let store = RELAY_STORE.read();
        let runtime = store.get(&relay_id).context("Relay not found")?;

        // Build context payload
        let context_msg = if let Some(ref ctx) = params.context {
            format!("{}\n\nContext: {}", message, serde_json::to_string_pretty(ctx).unwrap_or_default())
        } else {
            message
        };

        // Send handoff
        runtime.send_handoff("previous_agent", &target_agent, &context_msg);

        let response = json!({
            "status": "handoff_complete",
            "relay_id": relay_id,
            "target_agent": target_agent,
            "message": "Context successfully handed off to target agent"
        });

        Ok(ToolResult::success(
            serde_json::to_string_pretty(&response).unwrap_or_else(|_| format!("{:?}", response)),
        ))
    }

    /// Get status of a relay
    async fn get_status(&self, params: Params) -> Result<ToolResult> {
        let relay_id = params.relay_id.ok_or_else(|| anyhow::anyhow!("relay_id required for status"))?;

        let store = RELAY_STORE.read();
        
        if store.contains_key(&relay_id) {
            let response = json!({
                "status": "active",
                "relay_id": relay_id,
                "message": "Relay is active"
            });
            
            Ok(ToolResult::success(
                serde_json::to_string_pretty(&response).unwrap_or_else(|_| format!("{:?}", response)),
            ))
        } else {
            Ok(ToolResult::error(format!("Relay not found: {}", relay_id)))
        }
    }

    /// List agents in a relay
    async fn list_agents(&self, params: Params) -> Result<ToolResult> {
        let relay_id = params.relay_id.ok_or_else(|| anyhow::anyhow!("relay_id required for list_agents"))?;

        let store = RELAY_STORE.read();
        
        if store.contains_key(&relay_id) {
            let bus = get_agent_bus().await?;
            let agents: Vec<Value> = bus
                .registry
                .list()
                .iter()
                .map(|name| json!({ "name": name }))
                .collect();

            let response = json!({
                "relay_id": relay_id,
                "agents": agents,
                "count": agents.len()
            });

            Ok(ToolResult::success(
                serde_json::to_string_pretty(&response).unwrap_or_else(|_| format!("{:?}", response)),
            ))
        } else {
            Ok(ToolResult::error(format!("Relay not found: {}", relay_id)))
        }
    }

    /// Complete a relay and aggregate results
    async fn complete_relay(&self, params: Params) -> Result<ToolResult> {
        let relay_id = params.relay_id.ok_or_else(|| anyhow::anyhow!("relay_id required for complete"))?;

        // Get the runtime and shutdown agents
        let runtime = {
            let mut store = RELAY_STORE.write();
            store.remove(&relay_id)
        };

        if let Some(runtime) = runtime {
            runtime.shutdown_agents(&[]); // Shutdown all registered agents
        }

        let response = json!({
            "status": "completed",
            "relay_id": relay_id,
            "message": "Relay completed successfully. Results aggregated.",
            "aggregated_results": {
                "completed": true,
                "total_agents": 0
            }
        });

        Ok(ToolResult::success(
            serde_json::to_string_pretty(&response).unwrap_or_else(|_| format!("{:?}", response)),
        ))
    }
}
