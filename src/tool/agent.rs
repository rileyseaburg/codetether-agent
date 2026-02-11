//! Agent Tool - Spawn and communicate with sub-agents
//!
//! Allows the main agent to create specialized sub-agents, send them messages,
//! and receive their responses. Sub-agents maintain conversation history and
//! can use all available tools.

use super::{Tool, ToolResult};
use crate::provider::{ContentPart, ProviderRegistry, Role};
use crate::session::{Session, SessionEvent};
use anyhow::{Context, Result};
use async_trait::async_trait;
use parking_lot::RwLock;
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, OnceCell};

/// A spawned sub-agent with its own session and identity.
struct AgentEntry {
    instructions: String,
    session: Session,
}

lazy_static::lazy_static! {
    static ref AGENT_STORE: RwLock<HashMap<String, AgentEntry>> = RwLock::new(HashMap::new());
}

/// Lazily loaded provider registry — initialized on first agent interaction.
static PROVIDER_REGISTRY: OnceCell<Arc<ProviderRegistry>> = OnceCell::const_new();

async fn get_registry() -> Result<Arc<ProviderRegistry>> {
    let reg = PROVIDER_REGISTRY
        .get_or_try_init(|| async {
            let registry = ProviderRegistry::from_vault().await?;
            Ok::<_, anyhow::Error>(Arc::new(registry))
        })
        .await?;
    Ok(reg.clone())
}

pub struct AgentTool;

impl AgentTool {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Deserialize)]
struct Params {
    action: String,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    instructions: Option<String>,
    #[serde(default)]
    message: Option<String>,
    #[serde(default)]
    model: Option<String>,
}

#[async_trait]
impl Tool for AgentTool {
    fn id(&self) -> &str {
        "agent"
    }

    fn name(&self) -> &str {
        "Sub-Agent"
    }

    fn description(&self) -> &str {
        "Spawn and communicate with specialized sub-agents. Each sub-agent has its own conversation \
         history, system prompt, and access to all tools. Use this to delegate tasks to focused agents. \
         Actions: spawn (create agent), message (send message and get response), list (show agents), \
         kill (remove agent)."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["spawn", "message", "list", "kill"],
                    "description": "Action to perform"
                },
                "name": {
                    "type": "string",
                    "description": "Agent name (required for spawn, message, kill)"
                },
                "instructions": {
                    "type": "string",
                    "description": "System instructions for the agent (required for spawn). Describe the agent's role and expertise."
                },
                "message": {
                    "type": "string",
                    "description": "Message to send to the agent (required for message action)"
                },
                "model": {
                    "type": "string",
                    "description": "Model to use for the agent (optional, defaults to current model). Format: provider/model"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, params: Value) -> Result<ToolResult> {
        let p: Params = serde_json::from_value(params).context("Invalid params")?;

        match p.action.as_str() {
            "spawn" => {
                let name = p
                    .name
                    .ok_or_else(|| anyhow::anyhow!("name required for spawn"))?;
                let instructions = p
                    .instructions
                    .ok_or_else(|| anyhow::anyhow!("instructions required for spawn"))?;

                {
                    let store = AGENT_STORE.read();
                    if store.contains_key(&name) {
                        return Ok(ToolResult::error(format!(
                            "Agent @{name} already exists. Use kill first, or message it directly."
                        )));
                    }
                }

                let mut session = Session::new()
                    .await
                    .context("Failed to create session for sub-agent")?;

                session.agent = name.clone();
                if let Some(ref model) = p.model {
                    session.metadata.model = Some(model.clone());
                }

                session.add_message(crate::provider::Message {
                    role: Role::System,
                    content: vec![ContentPart::Text {
                        text: format!(
                            "You are @{name}, a specialized sub-agent. {instructions}\n\n\
                             You have access to all tools. Be thorough, focused, and concise. \
                             Complete the task fully before responding."
                        ),
                    }],
                });

                AGENT_STORE.write().insert(
                    name.clone(),
                    AgentEntry {
                        instructions: instructions.clone(),
                        session,
                    },
                );

                tracing::info!(agent = %name, "Sub-agent spawned");
                Ok(ToolResult::success(format!(
                    "Spawned agent @{name}: {instructions}\nSend it a message with action \"message\"."
                )))
            }

            "message" => {
                let name = p
                    .name
                    .ok_or_else(|| anyhow::anyhow!("name required for message"))?;
                let message = p
                    .message
                    .ok_or_else(|| anyhow::anyhow!("message required for message action"))?;

                // Take the session out of the store so we can mutably use it
                let mut session = {
                    let mut store = AGENT_STORE.write();
                    let entry = store
                        .get_mut(&name)
                        .ok_or_else(|| anyhow::anyhow!("Agent @{name} not found. Spawn it first."))?;
                    entry.session.clone()
                };

                let (tx, mut rx) = mpsc::channel::<SessionEvent>(256);
                let registry = get_registry().await?;

                // Run the agent's prompt loop
                let result = session.prompt_with_events(&message, tx, registry).await;

                // Collect the response
                let mut response_text = String::new();
                let mut thinking_text = String::new();
                let mut tool_calls = Vec::new();

                while let Ok(event) = rx.try_recv() {
                    match event {
                        SessionEvent::TextComplete(text) => {
                            response_text.push_str(&text);
                        }
                        SessionEvent::ThinkingComplete(text) => {
                            thinking_text.push_str(&text);
                        }
                        SessionEvent::ToolCallComplete {
                            name: tool_name,
                            output,
                            success,
                        } => {
                            tool_calls.push(json!({
                                "tool": tool_name,
                                "success": success,
                                "output_preview": if output.len() > 200 {
                                    format!("{}...", &output[..200])
                                } else {
                                    output
                                }
                            }));
                        }
                        SessionEvent::Error(err) => {
                            response_text.push_str(&format!("\n[Error: {err}]"));
                        }
                        _ => {}
                    }
                }

                // Put the updated session back
                {
                    let mut store = AGENT_STORE.write();
                    if let Some(entry) = store.get_mut(&name) {
                        entry.session = session;
                    }
                }

                if let Err(err) = result {
                    return Ok(ToolResult::error(format!(
                        "Agent @{name} failed: {err}"
                    )));
                }

                let mut output = json!({
                    "agent": name,
                    "response": response_text,
                });
                if !thinking_text.is_empty() {
                    output["thinking"] = json!(thinking_text);
                }
                if !tool_calls.is_empty() {
                    output["tool_calls"] = json!(tool_calls);
                }

                Ok(ToolResult::success(
                    serde_json::to_string_pretty(&output).unwrap_or(response_text),
                ))
            }

            "list" => {
                let store = AGENT_STORE.read();
                if store.is_empty() {
                    return Ok(ToolResult::success(
                        "No sub-agents spawned. Use action \"spawn\" to create one.",
                    ));
                }

                let agents: Vec<Value> = store
                    .iter()
                    .map(|(name, entry)| {
                        json!({
                            "name": name,
                            "instructions": entry.instructions,
                            "messages": entry.session.messages.len(),
                        })
                    })
                    .collect();

                Ok(ToolResult::success(
                    serde_json::to_string_pretty(&agents).unwrap_or_default(),
                ))
            }

            "kill" => {
                let name = p
                    .name
                    .ok_or_else(|| anyhow::anyhow!("name required for kill"))?;

                let removed = AGENT_STORE.write().remove(&name);
                match removed {
                    Some(_) => {
                        tracing::info!(agent = %name, "Sub-agent killed");
                        Ok(ToolResult::success(format!("Removed agent @{name}")))
                    }
                    None => Ok(ToolResult::error(format!("Agent @{name} not found"))),
                }
            }

            _ => Ok(ToolResult::error(format!(
                "Unknown action: {}. Valid: spawn, message, list, kill",
                p.action
            ))),
        }
    }
}
