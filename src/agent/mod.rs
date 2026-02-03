//! Agent system
//!
//! Agents are the core execution units that orchestrate tools and LLM interactions.

pub mod builtin;

use crate::config::PermissionAction;
use crate::provider::{CompletionRequest, Message, Provider, Role, ContentPart};
use crate::session::Session;
use crate::tool::{Tool, ToolRegistry, ToolResult};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Agent information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentInfo {
    pub name: String,
    pub description: Option<String>,
    pub mode: AgentMode,
    pub native: bool,
    pub hidden: bool,
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub max_steps: Option<usize>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AgentMode {
    Primary,
    Subagent,
    All,
}

/// The main agent execution context
pub struct Agent {
    pub info: AgentInfo,
    pub provider: Arc<dyn Provider>,
    pub tools: ToolRegistry,
    pub permissions: HashMap<String, PermissionAction>,
    system_prompt: String,
}

impl Agent {
    /// Create a new agent
    pub fn new(
        info: AgentInfo,
        provider: Arc<dyn Provider>,
        tools: ToolRegistry,
        system_prompt: String,
    ) -> Self {
        Self {
            info,
            provider,
            tools,
            permissions: HashMap::new(),
            system_prompt,
        }
    }

    /// Execute a prompt and return the response
    pub async fn execute(
        &self,
        session: &mut Session,
        prompt: &str,
    ) -> Result<AgentResponse> {
        // Add user message to session
        session.add_message(Message {
            role: Role::User,
            content: vec![ContentPart::Text { text: prompt.to_string() }],
        });

        let mut steps = 0;
        let max_steps = self.info.max_steps.unwrap_or(100);

        loop {
            steps += 1;
            if steps > max_steps {
                anyhow::bail!("Exceeded maximum steps ({})", max_steps);
            }

            // Build the completion request
            let request = CompletionRequest {
                messages: self.build_messages(session),
                tools: self.tools.definitions(),
                model: self.info.model.clone().unwrap_or_else(|| "gpt-4o".to_string()),
                temperature: self.info.temperature,
                top_p: self.info.top_p,
                max_tokens: None,
                stop: vec![],
            };

            // Get completion from provider
            let response = self.provider.complete(request).await?;
            session.add_message(response.message.clone());

            // Check for tool calls
            let tool_calls: Vec<_> = response
                .message
                .content
                .iter()
                .filter_map(|p| match p {
                    ContentPart::ToolCall { id, name, arguments } => {
                        Some((id.clone(), name.clone(), arguments.clone()))
                    }
                    _ => None,
                })
                .collect();

            if tool_calls.is_empty() {
                // No tool calls, we're done
                let text = response
                    .message
                    .content
                    .iter()
                    .filter_map(|p| match p {
                        ContentPart::Text { text } => Some(text.clone()),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                return Ok(AgentResponse {
                    text,
                    tool_uses: session.tool_uses.clone(),
                    usage: session.usage.clone(),
                });
            }

            // Execute tool calls
            for (id, name, arguments) in tool_calls {
                let result = self.execute_tool(&name, &arguments).await;
                
                session.tool_uses.push(ToolUse {
                    id: id.clone(),
                    name: name.clone(),
                    input: arguments.clone(),
                    output: result.output.clone(),
                    success: result.success,
                });

                session.add_message(Message {
                    role: Role::Tool,
                    content: vec![ContentPart::ToolResult {
                        tool_call_id: id,
                        content: result.output,
                    }],
                });
            }
        }
    }

    /// Build the full message list including system prompt
    fn build_messages(&self, session: &Session) -> Vec<Message> {
        let mut messages = vec![Message {
            role: Role::System,
            content: vec![ContentPart::Text {
                text: self.system_prompt.clone(),
            }],
        }];
        messages.extend(session.messages.clone());
        messages
    }

    /// Execute a single tool
    async fn execute_tool(&self, name: &str, arguments: &str) -> ToolResult {
        match self.tools.get(name) {
            Some(tool) => {
                let args: serde_json::Value = match serde_json::from_str(arguments) {
                    Ok(v) => v,
                    Err(e) => {
                        return ToolResult {
                            output: format!("Failed to parse arguments: {}", e),
                            success: false,
                            metadata: HashMap::new(),
                        }
                    }
                };
                
                match tool.execute(args).await {
                    Ok(result) => result,
                    Err(e) => ToolResult {
                        output: format!("Tool execution failed: {}", e),
                        success: false,
                        metadata: HashMap::new(),
                    },
                }
            }
            None => ToolResult {
                output: format!("Unknown tool: {}", name),
                success: false,
                metadata: HashMap::new(),
            },
        }
    }
}

/// Response from agent execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResponse {
    pub text: String,
    pub tool_uses: Vec<ToolUse>,
    pub usage: crate::provider::Usage,
}

/// Record of a tool use
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolUse {
    pub id: String,
    pub name: String,
    pub input: String,
    pub output: String,
    pub success: bool,
}

/// Registry of available agents
pub struct AgentRegistry {
    agents: HashMap<String, AgentInfo>,
}

impl AgentRegistry {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    /// Register a new agent
    pub fn register(&mut self, info: AgentInfo) {
        self.agents.insert(info.name.clone(), info);
    }

    /// Get agent info by name
    pub fn get(&self, name: &str) -> Option<&AgentInfo> {
        self.agents.get(name)
    }

    /// List all agents
    pub fn list(&self) -> Vec<&AgentInfo> {
        self.agents.values().collect()
    }

    /// List primary agents (visible in UI)
    pub fn list_primary(&self) -> Vec<&AgentInfo> {
        self.agents
            .values()
            .filter(|a| a.mode == AgentMode::Primary && !a.hidden)
            .collect()
    }

    /// Initialize with builtin agents
    pub fn with_builtins() -> Self {
        let mut registry = Self::new();
        
        registry.register(builtin::build_agent());
        registry.register(builtin::plan_agent());
        registry.register(builtin::explore_agent());
        
        registry
    }
}

impl Default for AgentRegistry {
    fn default() -> Self {
        Self::with_builtins()
    }
}
