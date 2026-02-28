//! Agent Tool - Spawn and communicate with sub-agents
//!
//! Allows the main agent to create specialized sub-agents, send them messages,
//! and receive their responses. Sub-agents maintain conversation history and
//! can use all available tools.

use super::{Tool, ToolResult};
use crate::provider::{ContentPart, ProviderRegistry, Role, parse_model_string};
use crate::session::{Session, SessionEvent};
use anyhow::{Context, Result};
use async_trait::async_trait;
use parking_lot::RwLock;
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{OnceCell, mpsc};

/// A spawned sub-agent with its own session and identity.
struct AgentEntry {
    instructions: String,
    session: Session,
}

#[derive(Clone)]
pub struct AgentSnapshot {
    pub name: String,
    pub instructions: String,
    pub session: Session,
}

lazy_static::lazy_static! {
    static ref AGENT_STORE: RwLock<HashMap<String, AgentEntry>> = RwLock::new(HashMap::new());
}

/// Lazily loaded provider registry — initialized on first agent interaction.
static PROVIDER_REGISTRY: OnceCell<Arc<ProviderRegistry>> = OnceCell::const_new();

pub fn list_agent_snapshots() -> Vec<AgentSnapshot> {
    AGENT_STORE
        .read()
        .iter()
        .map(|(name, entry)| AgentSnapshot {
            name: name.clone(),
            instructions: entry.instructions.clone(),
            session: entry.session.clone(),
        })
        .collect()
}

pub fn remove_agent(name: &str) -> bool {
    AGENT_STORE.write().remove(name).is_some()
}

async fn get_registry() -> Result<Arc<ProviderRegistry>> {
    let reg = PROVIDER_REGISTRY
        .get_or_try_init(|| async {
            let registry = ProviderRegistry::from_vault().await?;
            Ok::<_, anyhow::Error>(Arc::new(registry))
        })
        .await?;
    Ok(reg.clone())
}

fn truncate_preview(input: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }

    let mut chars = input.chars();
    let preview: String = chars.by_ref().take(max_chars).collect();
    if chars.next().is_some() {
        format!("{preview}...")
    } else {
        preview
    }
}

fn normalize_model_ref(model: &str) -> String {
    model.trim().to_ascii_lowercase()
}

fn is_subscription_or_included_provider(provider: &str) -> bool {
    matches!(
        provider,
        "openai-codex"
            | "github-copilot"
            | "github-copilot-enterprise"
            | "gemini-web"
            | "local_cuda"
    )
}

fn is_explicit_free_model_id(model_id: &str) -> bool {
    let lowered = model_id.to_ascii_lowercase();
    lowered.contains(":free") || lowered.ends_with("-free")
}

async fn is_free_or_subscription_model(model_ref: &str, registry: &ProviderRegistry) -> bool {
    let trimmed = model_ref.trim();
    if trimmed.is_empty() {
        return false;
    }

    let (provider_opt, model_id) = parse_model_string(trimmed);
    let Some(provider_name) = provider_opt else {
        return is_explicit_free_model_id(trimmed);
    };

    let provider_norm = provider_name.to_ascii_lowercase();
    if is_subscription_or_included_provider(&provider_norm) || is_explicit_free_model_id(model_id) {
        return true;
    }

    let Some(provider) = registry.get(provider_name) else {
        return false;
    };

    let Ok(models) = provider.list_models().await else {
        return false;
    };

    models.into_iter().any(|m| {
        if !m.id.eq_ignore_ascii_case(model_id) {
            return false;
        }
        let input = m.input_cost_per_million.unwrap_or(1.0);
        let output = m.output_cost_per_million.unwrap_or(1.0);
        input <= 0.0 && output <= 0.0
    })
}

pub struct AgentTool;

impl Default for AgentTool {
    fn default() -> Self {
        Self::new()
    }
}

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
    #[serde(default, rename = "__ct_current_model")]
    current_model: Option<String>,
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
                    "description": "Model to use for the agent. Required for spawn. Must be provider/model and must be different from the caller model."
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
                let requested_model = p.model.ok_or_else(|| {
                    anyhow::anyhow!(
                        "model required for spawn. Policy: spawned agents must use a different free/subscription model."
                    )
                })?;
                let current_model = p
                    .current_model
                    .or_else(|| std::env::var("CODETETHER_DEFAULT_MODEL").ok())
                    .ok_or_else(|| {
                        anyhow::anyhow!(
                            "cannot determine caller model for spawn policy. Set an explicit model before spawning."
                        )
                    })?;

                if normalize_model_ref(&requested_model) == normalize_model_ref(&current_model) {
                    return Ok(ToolResult::error(format!(
                        "Spawn blocked: requested model '{requested_model}' must be different from caller model '{current_model}'."
                    )));
                }

                let registry = get_registry().await?;
                if !is_free_or_subscription_model(&requested_model, &registry).await {
                    return Ok(ToolResult::error(format!(
                        "Spawn blocked: model '{requested_model}' is not free/subscription-eligible. \
                         Use a free tier (e.g. ':free') or subscription/included provider model."
                    )));
                }

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
                session.metadata.model = Some(requested_model.clone());

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
                    "Spawned agent @{name} on model '{requested_model}': {instructions}\nSend it a message with action \"message\"."
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
                    let entry = store.get_mut(&name).ok_or_else(|| {
                        anyhow::anyhow!("Agent @{name} not found. Spawn it first.")
                    })?;
                    entry.session.clone()
                };

                let (tx, mut rx) = mpsc::channel::<SessionEvent>(256);
                let registry = get_registry().await?;

                // Use tokio::spawn to run the agent in the background
                // This allows the main event loop to remain responsive
                let mut session_clone = session.clone();
                let msg_clone = message.clone();
                let registry_clone = registry.clone();
                let tx_clone = tx.clone();

                // Spawn the agent prompt task
                let handle = tokio::spawn(async move {
                    session_clone
                        .prompt_with_events(&msg_clone, tx_clone, registry_clone)
                        .await
                });

                // Wait for completion events with yielding to stay responsive
                let mut response_text = String::new();
                let mut thinking_text = String::new();
                let mut tool_calls = Vec::new();
                let mut agent_done = false;
                let mut last_error: Option<String> = None;

                // Maximum wait time: 5 minutes per agent
                let max_wait = std::time::Duration::from_secs(300);
                let start = std::time::Instant::now();

                while !agent_done && start.elapsed() < max_wait {
                    // Yield immediately on each iteration to keep the TUI event loop responsive
                    // This prevents UI freezes when multiple agents are spawned
                    tokio::task::yield_now().await;

                    // Use tokio::select! with small timeout to stay responsive
                    match tokio::time::timeout(std::time::Duration::from_millis(20), rx.recv())
                        .await
                    {
                        Ok(Some(event)) => match event {
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
                                    "output_preview": truncate_preview(&output, 200)
                                }));
                            }
                            SessionEvent::Error(err) => {
                                response_text.push_str(&format!("\n[Error: {err}]"));
                                last_error = Some(err);
                            }
                            SessionEvent::Done => {
                                agent_done = true;
                            }
                            SessionEvent::SessionSync(synced) => {
                                session = *synced;
                            }
                            _ => {}
                        },
                        Ok(None) => {
                            // Channel closed
                            agent_done = true;
                        }
                        Err(_) => {
                            // Timeout - check if spawn is done and yield to other tasks
                            if handle.is_finished() {
                                agent_done = true;
                            }
                        }
                    }
                }

                // Check the result of the spawned task
                if handle.is_finished() {
                    match handle.await {
                        Ok(Ok(_)) => {}
                        Ok(Err(err)) => {
                            if last_error.is_none() {
                                last_error = Some(err.to_string());
                            }
                        }
                        Err(err) => {
                            if err.is_cancelled() {
                                last_error = Some("Agent task was cancelled".to_string());
                            } else {
                                last_error = Some(format!("Agent task panicked: {}", err));
                            }
                        }
                    }
                } else {
                    // Agent didn't finish in time - abort it
                    handle.abort();
                    if last_error.is_none() {
                        last_error = Some(format!("Agent @{name} timed out after 5 minutes"));
                    }
                }

                // Put the updated session back
                {
                    let mut store = AGENT_STORE.write();
                    if let Some(entry) = store.get_mut(&name) {
                        entry.session = session;
                    }
                }

                if let Some(ref err) = last_error {
                    if response_text.is_empty() {
                        return Ok(ToolResult::error(format!("Agent @{name} failed: {err}")));
                    }
                    // Partial response with error
                    response_text.push_str(&format!("\n\n[Warning: {err}]"));
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

#[cfg(test)]
mod tests {
    use super::{
        is_explicit_free_model_id, is_free_or_subscription_model, normalize_model_ref,
        truncate_preview,
    };
    use crate::provider::ProviderRegistry;

    #[test]
    fn truncate_preview_respects_char_boundaries() {
        let input = "a".repeat(198) + "───";
        let output = truncate_preview(&input, 200);
        assert!(output.ends_with("..."));
        assert!(output.starts_with(&"a".repeat(198)));
    }

    #[test]
    fn truncate_preview_returns_original_when_short_enough() {
        let input = "hello world";
        assert_eq!(truncate_preview(input, 200), input);
    }

    #[test]
    fn normalize_model_ref_trims_and_lowercases() {
        assert_eq!(
            normalize_model_ref("  OpenAI-Codex/GPT-5.1-Codex  "),
            "openai-codex/gpt-5.1-codex"
        );
    }

    #[test]
    fn free_model_id_detection_handles_openrouter_suffix() {
        assert!(is_explicit_free_model_id("z-ai/glm-5:free"));
        assert!(is_explicit_free_model_id("kimi-k2-free"));
        assert!(!is_explicit_free_model_id("gpt-4.1"));
    }

    #[tokio::test]
    async fn free_or_subscription_model_accepts_known_included_provider() {
        let registry = ProviderRegistry::new();
        assert!(is_free_or_subscription_model("openai-codex/gpt-5-mini", &registry).await);
    }

    #[tokio::test]
    async fn free_or_subscription_model_rejects_unknown_paid_model_without_metadata() {
        let registry = ProviderRegistry::new();
        assert!(
            !is_free_or_subscription_model("anthropic/claude-sonnet-4-20250514", &registry).await
        );
    }
}
