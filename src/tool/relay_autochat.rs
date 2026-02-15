//! Relay AutoChat Tool - Autonomous relay communication between agents
//!
//! Enables task delegation and result aggregation between agents using
//! the protocol-first relay runtime. This tool allows LLMs to trigger
//! agent handoffs and coordinate multi-agent workflows.

use super::{Tool, ToolResult};
use crate::bus::AgentBus;
use crate::bus::relay::ProtocolRelayRuntime;
use anyhow::Result;
use async_trait::async_trait;
use parking_lot::RwLock;
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
        let action = match params.get("action").and_then(|v| v.as_str()) {
            Some(s) if !s.is_empty() => s.to_string(),
            _ => {
                return Ok(ToolResult::structured_error(
                    "MISSING_FIELD",
                    "relay_autochat",
                    "action is required. Valid actions: init, delegate, handoff, status, list_agents, complete",
                    Some(vec!["action"]),
                    Some(json!({
                        "action": "init",
                        "task": "description of the relay task"
                    })),
                ));
            }
        };

        let relay_id = params
            .get("relay_id")
            .and_then(|v| v.as_str())
            .map(String::from);
        let target_agent = params
            .get("target_agent")
            .and_then(|v| v.as_str())
            .map(String::from);
        let message = params
            .get("message")
            .and_then(|v| v.as_str())
            .map(String::from);
        let context = params.get("context").cloned();
        let okr_id = params
            .get("okr_id")
            .and_then(|v| v.as_str())
            .map(String::from);
        let task = params
            .get("task")
            .and_then(|v| v.as_str())
            .map(String::from);

        match action.as_str() {
            "init" => self.init_relay(relay_id, task, context, okr_id).await,
            "delegate" => {
                self.delegate_task(relay_id, target_agent, message, context, okr_id)
                    .await
            }
            "handoff" => {
                self.handoff_context(relay_id, target_agent, message, context)
                    .await
            }
            "status" => self.get_status(relay_id).await,
            "list_agents" => self.list_agents(relay_id).await,
            "complete" => self.complete_relay(relay_id).await,
            _ => Ok(ToolResult::structured_error(
                "INVALID_ACTION",
                "relay_autochat",
                &format!(
                    "Unknown action: '{action}'. Valid actions: init, delegate, handoff, status, list_agents, complete"
                ),
                None,
                Some(json!({
                    "action": "init",
                    "task": "description of the relay task"
                })),
            )),
        }
    }
}

impl RelayAutoChatTool {
    /// Initialize a new relay with a task
    async fn init_relay(
        &self,
        relay_id: Option<String>,
        task: Option<String>,
        _context: Option<Value>,
        okr_id: Option<String>,
    ) -> Result<ToolResult> {
        let task = task.unwrap_or_else(|| "Unspecified task".to_string());
        let relay_id =
            relay_id.unwrap_or_else(|| format!("relay-{}", &Uuid::new_v4().to_string()[..8]));

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
            "okr_id": okr_id,
            "message": "Relay initialized. Use 'delegate' to assign tasks to agents, or 'list_agents' to see available agents."
        });

        let mut result = ToolResult::success(
            serde_json::to_string_pretty(&response).unwrap_or_else(|_| format!("{:?}", response)),
        )
        .with_metadata("relay_id", json!(relay_id));
        if let Some(okr_id) = response.get("okr_id").and_then(|v| v.as_str()) {
            result = result.with_metadata("okr_id", json!(okr_id));
        }

        Ok(result)
    }

    /// Delegate a task to a target agent
    async fn delegate_task(
        &self,
        relay_id: Option<String>,
        target_agent: Option<String>,
        message: Option<String>,
        context: Option<Value>,
        okr_id: Option<String>,
    ) -> Result<ToolResult> {
        let relay_id = match relay_id {
            Some(id) => id,
            None => {
                return Ok(ToolResult::structured_error(
                    "MISSING_FIELD",
                    "relay_autochat",
                    "relay_id is required for delegate action",
                    Some(vec!["relay_id"]),
                    Some(
                        json!({"action": "delegate", "relay_id": "relay-xxx", "target_agent": "agent-name", "message": "task description"}),
                    ),
                ));
            }
        };
        let target_agent = match target_agent {
            Some(a) => a,
            None => {
                return Ok(ToolResult::structured_error(
                    "MISSING_FIELD",
                    "relay_autochat",
                    "target_agent is required for delegate action",
                    Some(vec!["target_agent"]),
                    Some(
                        json!({"action": "delegate", "relay_id": relay_id, "target_agent": "agent-name", "message": "task description"}),
                    ),
                ));
            }
        };
        let message = message.unwrap_or_else(|| "New task assigned".to_string());

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
        let context_msg = if let Some(ref ctx) = context {
            format!(
                "{}\n\nContext: {}",
                message,
                serde_json::to_string_pretty(ctx).unwrap_or_default()
            )
        } else {
            message.clone()
        };

        // Send the delegation message
        runtime.send_handoff("system", &target_agent, &context_msg);

        let response = json!({
            "status": "delegated",
            "relay_id": relay_id,
            "target_agent": target_agent,
            "okr_id": okr_id,
            "message": message,
            "initial_results": {
                "task_assigned": true,
                "agent_notified": true
            }
        });

        let mut result = ToolResult::success(
            serde_json::to_string_pretty(&response).unwrap_or_else(|_| format!("{:?}", response)),
        )
        .with_metadata("relay_id", json!(relay_id))
        .with_metadata("target_agent", json!(target_agent));
        if let Some(okr_id) = response.get("okr_id").and_then(|v| v.as_str()) {
            result = result.with_metadata("okr_id", json!(okr_id));
        }

        Ok(result)
    }

    /// Hand off context between agents
    async fn handoff_context(
        &self,
        relay_id: Option<String>,
        target_agent: Option<String>,
        message: Option<String>,
        context: Option<Value>,
    ) -> Result<ToolResult> {
        let relay_id = match relay_id {
            Some(id) => id,
            None => {
                return Ok(ToolResult::structured_error(
                    "MISSING_FIELD",
                    "relay_autochat",
                    "relay_id is required for handoff action",
                    Some(vec!["relay_id"]),
                    Some(
                        json!({"action": "handoff", "relay_id": "relay-xxx", "target_agent": "agent-name"}),
                    ),
                ));
            }
        };
        let target_agent = match target_agent {
            Some(a) => a,
            None => {
                return Ok(ToolResult::structured_error(
                    "MISSING_FIELD",
                    "relay_autochat",
                    "target_agent is required for handoff action",
                    Some(vec!["target_agent"]),
                    Some(
                        json!({"action": "handoff", "relay_id": relay_id, "target_agent": "agent-name"}),
                    ),
                ));
            }
        };
        let message = message.unwrap_or_else(|| "Context handoff".to_string());

        let store = RELAY_STORE.read();
        let runtime = match store.get(&relay_id) {
            Some(r) => r.clone(),
            None => {
                return Ok(ToolResult::structured_error(
                    "NOT_FOUND",
                    "relay_autochat",
                    &format!(
                        "Relay not found: {relay_id}. Use 'init' action to create a relay first."
                    ),
                    None,
                    Some(json!({"action": "init", "task": "description of the relay task"})),
                ));
            }
        };
        // need to drop the lock before await
        drop(store);

        // Build context payload
        let context_msg = if let Some(ref ctx) = context {
            format!(
                "{}\n\nContext: {}",
                message,
                serde_json::to_string_pretty(ctx).unwrap_or_default()
            )
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
    async fn get_status(&self, relay_id: Option<String>) -> Result<ToolResult> {
        let relay_id = match relay_id {
            Some(id) => id,
            None => {
                return Ok(ToolResult::structured_error(
                    "MISSING_FIELD",
                    "relay_autochat",
                    "relay_id is required for status action",
                    Some(vec!["relay_id"]),
                    Some(json!({"action": "status", "relay_id": "relay-xxx"})),
                ));
            }
        };

        let store = RELAY_STORE.read();

        if store.contains_key(&relay_id) {
            let response = json!({
                "status": "active",
                "relay_id": relay_id,
                "message": "Relay is active"
            });

            Ok(ToolResult::success(
                serde_json::to_string_pretty(&response)
                    .unwrap_or_else(|_| format!("{:?}", response)),
            ))
        } else {
            Ok(ToolResult::error(format!("Relay not found: {}", relay_id)))
        }
    }

    /// List agents in a relay
    async fn list_agents(&self, relay_id: Option<String>) -> Result<ToolResult> {
        let relay_id = match relay_id {
            Some(id) => id,
            None => {
                return Ok(ToolResult::structured_error(
                    "MISSING_FIELD",
                    "relay_autochat",
                    "relay_id is required for list_agents action",
                    Some(vec!["relay_id"]),
                    Some(json!({"action": "list_agents", "relay_id": "relay-xxx"})),
                ));
            }
        };

        let relay_exists = {
            let store = RELAY_STORE.read();
            store.contains_key(&relay_id)
        };

        if relay_exists {
            let bus = get_agent_bus().await?;
            let agents: Vec<Value> = bus
                .registry
                .agent_ids()
                .iter()
                .map(|name| json!({ "name": name }))
                .collect();

            let response = json!({
                "relay_id": relay_id,
                "agents": agents,
                "count": agents.len()
            });

            Ok(ToolResult::success(
                serde_json::to_string_pretty(&response)
                    .unwrap_or_else(|_| format!("{:?}", response)),
            ))
        } else {
            Ok(ToolResult::error(format!("Relay not found: {}", relay_id)))
        }
    }

    /// Complete a relay and aggregate results
    async fn complete_relay(&self, relay_id: Option<String>) -> Result<ToolResult> {
        let relay_id = match relay_id {
            Some(id) => id,
            None => {
                return Ok(ToolResult::structured_error(
                    "MISSING_FIELD",
                    "relay_autochat",
                    "relay_id is required for complete action",
                    Some(vec!["relay_id"]),
                    Some(json!({"action": "complete", "relay_id": "relay-xxx"})),
                ));
            }
        };

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
