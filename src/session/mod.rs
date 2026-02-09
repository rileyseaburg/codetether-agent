//! Session management
//!
//! Sessions track the conversation history and state for agent interactions.

use crate::agent::ToolUse;
use crate::provider::{Message, Usage};
use crate::tool::ToolRegistry;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use uuid::Uuid;

#[cfg(feature = "functiongemma")]
use crate::cognition::tool_router::{ToolCallRouter, ToolRouterConfig};

fn is_interactive_tool(tool_name: &str) -> bool {
    matches!(tool_name, "question")
}

/// A conversation session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub title: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub messages: Vec<Message>,
    pub tool_uses: Vec<ToolUse>,
    pub usage: Usage,
    pub agent: String,
    pub metadata: SessionMetadata,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionMetadata {
    pub directory: Option<PathBuf>,
    pub model: Option<String>,
    pub shared: bool,
    pub share_url: Option<String>,
}

impl Session {
    fn default_model_for_provider(provider: &str) -> String {
        match provider {
            "moonshotai" => "kimi-k2.5".to_string(),
            "anthropic" => "claude-sonnet-4-20250514".to_string(),
            "openai" => "gpt-4o".to_string(),
            "google" => "gemini-2.5-pro".to_string(),
            "zhipuai" => "glm-4.7".to_string(),
            "openrouter" => "z-ai/glm-4.7".to_string(),
            "novita" => "qwen/qwen3-coder-next".to_string(),
            "github-copilot" | "github-copilot-enterprise" => "gpt-5-mini".to_string(),
            _ => "glm-4.7".to_string(),
        }
    }

    /// Create a new session
    pub async fn new() -> Result<Self> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();

        Ok(Self {
            id,
            title: None,
            created_at: now,
            updated_at: now,
            messages: Vec::new(),
            tool_uses: Vec::new(),
            usage: Usage::default(),
            agent: "build".to_string(),
            metadata: SessionMetadata {
                directory: Some(std::env::current_dir()?),
                ..Default::default()
            },
        })
    }

    /// Load an existing session
    pub async fn load(id: &str) -> Result<Self> {
        let path = Self::session_path(id)?;
        let content = fs::read_to_string(&path).await?;
        let session: Session = serde_json::from_str(&content)?;
        Ok(session)
    }

    /// Load the last session, optionally scoped to a workspace directory
    ///
    /// When `workspace` is Some, only considers sessions created in that directory.
    /// When None, returns the most recent session globally (legacy behavior).
    pub async fn last_for_directory(workspace: Option<&std::path::Path>) -> Result<Self> {
        let sessions_dir = Self::sessions_dir()?;

        if !sessions_dir.exists() {
            anyhow::bail!("No sessions found");
        }

        let mut entries: Vec<tokio::fs::DirEntry> = Vec::new();
        let mut read_dir = fs::read_dir(&sessions_dir).await?;
        while let Some(entry) = read_dir.next_entry().await? {
            entries.push(entry);
        }

        if entries.is_empty() {
            anyhow::bail!("No sessions found");
        }

        // Sort by modification time (most recent first)
        // Use std::fs::metadata since we can't await in sort_by_key
        entries.sort_by_key(|e| {
            std::cmp::Reverse(
                std::fs::metadata(e.path())
                    .ok()
                    .and_then(|m| m.modified().ok())
                    .unwrap_or(std::time::SystemTime::UNIX_EPOCH),
            )
        });

        let canonical_workspace = workspace.map(|w| w.canonicalize().unwrap_or_else(|_| w.to_path_buf()));

        for entry in &entries {
            let content: String = fs::read_to_string(entry.path()).await?;
            if let Ok(session) = serde_json::from_str::<Session>(&content) {
                // If workspace scoping requested, filter by directory
                if let Some(ref ws) = canonical_workspace {
                    if let Some(ref dir) = session.metadata.directory {
                        let canonical_dir = dir.canonicalize().unwrap_or_else(|_| dir.clone());
                        if &canonical_dir == ws {
                            return Ok(session);
                        }
                    }
                    continue;
                }
                return Ok(session);
            }
        }

        anyhow::bail!("No sessions found")
    }

    /// Load the last session (global, unscoped — legacy compatibility)
    pub async fn last() -> Result<Self> {
        Self::last_for_directory(None).await
    }

    /// Save the session to disk
    pub async fn save(&self) -> Result<()> {
        let path = Self::session_path(&self.id)?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content).await?;

        Ok(())
    }

    /// Add a message to the session
    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        self.updated_at = Utc::now();
    }

    /// Execute a prompt and get the result
    pub async fn prompt(&mut self, message: &str) -> Result<SessionResult> {
        use crate::provider::{
            CompletionRequest, ContentPart, ProviderRegistry, Role, parse_model_string,
        };

        // Load providers from Vault
        let registry = ProviderRegistry::from_vault().await?;

        let providers = registry.list();
        if providers.is_empty() {
            anyhow::bail!(
                "No providers available. Configure API keys in HashiCorp Vault (for Copilot use `codetether auth copilot`)."
            );
        }

        tracing::info!("Available providers: {:?}", providers);

        // Parse model string (format: "provider/model", "provider", or just "model")
        let (provider_name, model_id) = if let Some(ref model_str) = self.metadata.model {
            let (prov, model) = parse_model_string(model_str);
            if prov.is_some() {
                // Format: provider/model
                (prov.map(|s| s.to_string()), model.to_string())
            } else if providers.contains(&model) {
                // Format: just provider name (e.g., "novita")
                (Some(model.to_string()), String::new())
            } else {
                // Format: just model name
                (None, model.to_string())
            }
        } else {
            (None, String::new())
        };

        // Determine which provider to use (prefer zhipuai as default)
        let selected_provider = provider_name
            .as_deref()
            .filter(|p| providers.contains(p))
            .or_else(|| {
                if providers.contains(&"zhipuai") {
                    Some("zhipuai")
                } else {
                    providers.first().copied()
                }
            })
            .ok_or_else(|| anyhow::anyhow!("No providers available"))?;

        let provider = registry
            .get(selected_provider)
            .ok_or_else(|| anyhow::anyhow!("Provider {} not found", selected_provider))?;

        // Add user message to session using add_message
        self.add_message(Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: message.to_string(),
            }],
        });

        // Generate title if this is the first user message and no title exists
        if self.title.is_none() {
            self.generate_title().await?;
        }

        // Determine model to use
        let model = if !model_id.is_empty() {
            model_id
        } else {
            Self::default_model_for_provider(selected_provider)
        };

        // Create tool registry with all available tools
        let tool_registry = ToolRegistry::with_provider_arc(Arc::clone(&provider), model.clone());
        let tool_definitions: Vec<_> = tool_registry
            .definitions()
            .into_iter()
            .filter(|tool| !is_interactive_tool(&tool.name))
            .collect();

        // Kimi K2.5 requires temperature=1.0
        let temperature = if model.starts_with("kimi-k2") {
            Some(1.0)
        } else {
            Some(0.7)
        };

        tracing::info!("Using model: {} via provider: {}", model, selected_provider);
        tracing::info!("Available tools: {}", tool_definitions.len());

        // Check whether the model natively supports tool calling.
        // If it does, FunctionGemma is unnecessary and will be skipped.
        #[cfg(feature = "functiongemma")]
        let model_supports_tools = provider
            .list_models()
            .await
            .unwrap_or_default()
            .iter()
            .find(|m| m.id == model)
            .map(|m| m.supports_tools)
            .unwrap_or(true); // Default true = assume native support (safe).

        // Build system prompt with AGENTS.md
        let cwd = self
            .metadata
            .directory
            .clone()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
        let system_prompt = crate::agent::builtin::build_system_prompt(&cwd);

        // Run agentic loop with tool execution
        let max_steps = 50;
        let mut final_output = String::new();

        // Initialise the FunctionGemma tool-call router (feature-gated, opt-in).
        #[cfg(feature = "functiongemma")]
        let tool_router: Option<ToolCallRouter> = {
            let cfg = ToolRouterConfig::from_env();
            match ToolCallRouter::from_config(&cfg) {
                Ok(r) => r,
                Err(e) => {
                    tracing::warn!(error = %e, "FunctionGemma tool router init failed; disabled");
                    None
                }
            }
        };

        for step in 1..=max_steps {
            tracing::info!(step = step, "Agent step starting");

            // Build messages with system prompt first
            let mut messages = vec![Message {
                role: Role::System,
                content: vec![ContentPart::Text {
                    text: system_prompt.clone(),
                }],
            }];
            messages.extend(self.messages.clone());

            // Create completion request with tools
            let request = CompletionRequest {
                messages,
                tools: tool_definitions.clone(),
                model: model.clone(),
                temperature,
                top_p: None,
                max_tokens: Some(8192),
                stop: Vec::new(),
            };

            // Call the provider
            let response = provider.complete(request).await?;

            // Optionally route text-only responses through FunctionGemma to
            // produce structured tool calls.  Skipped when the model natively
            // supports tool calling (which all current providers do).
            #[cfg(feature = "functiongemma")]
            let response = if let Some(ref router) = tool_router {
                router
                    .maybe_reformat(response, &tool_definitions, model_supports_tools)
                    .await
            } else {
                response
            };

            // Record token usage
            crate::telemetry::TOKEN_USAGE.record_model_usage(
                &model,
                response.usage.prompt_tokens as u64,
                response.usage.completion_tokens as u64,
            );

            // Extract tool calls from response
            let tool_calls: Vec<(String, String, serde_json::Value)> = response
                .message
                .content
                .iter()
                .filter_map(|part| {
                    if let ContentPart::ToolCall {
                        id,
                        name,
                        arguments,
                    } = part
                    {
                        // Parse arguments JSON string into Value
                        let args: serde_json::Value =
                            serde_json::from_str(arguments).unwrap_or(serde_json::json!({}));
                        Some((id.clone(), name.clone(), args))
                    } else {
                        None
                    }
                })
                .collect();

            // Collect text output
            for part in &response.message.content {
                if let ContentPart::Text { text } = part {
                    if !text.is_empty() {
                        final_output.push_str(text);
                        final_output.push('\n');
                    }
                }
            }

            // If no tool calls, we're done
            if tool_calls.is_empty() {
                self.add_message(response.message.clone());
                break;
            }

            // Add assistant message with tool calls
            self.add_message(response.message.clone());

            tracing::info!(
                step = step,
                num_tools = tool_calls.len(),
                "Executing tool calls"
            );

            // Execute each tool call
            for (tool_id, tool_name, tool_input) in tool_calls {
                tracing::info!(tool = %tool_name, tool_id = %tool_id, "Executing tool");

                if is_interactive_tool(&tool_name) {
                    tracing::warn!(tool = %tool_name, "Blocking interactive tool in session loop");
                    self.add_message(Message {
                        role: Role::Tool,
                        content: vec![ContentPart::ToolResult {
                            tool_call_id: tool_id,
                            content: "Error: Interactive tool 'question' is disabled in this interface. Ask the user directly in assistant text.".to_string(),
                        }],
                    });
                    continue;
                }

                // Get and execute the tool
                let content = if let Some(tool) = tool_registry.get(&tool_name) {
                    match tool.execute(tool_input.clone()).await {
                        Ok(result) => {
                            tracing::info!(tool = %tool_name, success = result.success, "Tool execution completed");
                            result.output
                        }
                        Err(e) => {
                            tracing::warn!(tool = %tool_name, error = %e, "Tool execution failed");
                            format!("Error: {}", e)
                        }
                    }
                } else {
                    tracing::warn!(tool = %tool_name, "Tool not found");
                    format!("Error: Unknown tool '{}'", tool_name)
                };

                // Add tool result message
                self.add_message(Message {
                    role: Role::Tool,
                    content: vec![ContentPart::ToolResult {
                        tool_call_id: tool_id,
                        content,
                    }],
                });
            }
        }

        // Save session after each prompt to persist messages
        self.save().await?;

        Ok(SessionResult {
            text: final_output.trim().to_string(),
            session_id: self.id.clone(),
        })
    }

    /// Process a user message with real-time event streaming for UI updates.
    /// Events are sent through the provided channel as tool calls execute.
    pub async fn prompt_with_events(
        &mut self,
        message: &str,
        event_tx: tokio::sync::mpsc::Sender<SessionEvent>,
    ) -> Result<SessionResult> {
        use crate::provider::{
            CompletionRequest, ContentPart, ProviderRegistry, Role, parse_model_string,
        };

        let _ = event_tx.send(SessionEvent::Thinking).await;

        // Load provider registry from Vault
        let registry = ProviderRegistry::from_vault().await?;
        let providers = registry.list();
        if providers.is_empty() {
            anyhow::bail!(
                "No providers available. Configure API keys in HashiCorp Vault (for Copilot use `codetether auth copilot`)."
            );
        }
        tracing::info!("Available providers: {:?}", providers);

        // Parse model string (format: "provider/model", "provider", or just "model")
        let (provider_name, model_id) = if let Some(ref model_str) = self.metadata.model {
            let (prov, model) = parse_model_string(model_str);
            if prov.is_some() {
                (prov.map(|s| s.to_string()), model.to_string())
            } else if providers.contains(&model) {
                (Some(model.to_string()), String::new())
            } else {
                (None, model.to_string())
            }
        } else {
            (None, String::new())
        };

        // Determine which provider to use (prefer zhipuai as default)
        let selected_provider = provider_name
            .as_deref()
            .filter(|p| providers.contains(p))
            .or_else(|| {
                if providers.contains(&"zhipuai") {
                    Some("zhipuai")
                } else {
                    providers.first().copied()
                }
            })
            .ok_or_else(|| anyhow::anyhow!("No providers available"))?;

        let provider = registry
            .get(selected_provider)
            .ok_or_else(|| anyhow::anyhow!("Provider {} not found", selected_provider))?;

        // Add user message
        self.add_message(Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: message.to_string(),
            }],
        });

        // Generate title if needed
        if self.title.is_none() {
            self.generate_title().await?;
        }

        // Determine model
        let model = if !model_id.is_empty() {
            model_id
        } else {
            Self::default_model_for_provider(selected_provider)
        };

        // Create tool registry
        let tool_registry = ToolRegistry::with_provider_arc(Arc::clone(&provider), model.clone());
        let tool_definitions: Vec<_> = tool_registry
            .definitions()
            .into_iter()
            .filter(|tool| !is_interactive_tool(&tool.name))
            .collect();

        let temperature = if model.starts_with("kimi-k2") {
            Some(1.0)
        } else {
            Some(0.7)
        };

        tracing::info!("Using model: {} via provider: {}", model, selected_provider);
        tracing::info!("Available tools: {}", tool_definitions.len());

        // Check whether the model natively supports tool calling.
        #[cfg(feature = "functiongemma")]
        let model_supports_tools = provider
            .list_models()
            .await
            .unwrap_or_default()
            .iter()
            .find(|m| m.id == model)
            .map(|m| m.supports_tools)
            .unwrap_or(true);

        // Build system prompt
        let cwd = std::env::var("PWD")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default());
        let system_prompt = crate::agent::builtin::build_system_prompt(&cwd);

        let mut final_output = String::new();
        let max_steps = 50;

        // Initialise the FunctionGemma tool-call router (feature-gated, opt-in).
        #[cfg(feature = "functiongemma")]
        let tool_router: Option<ToolCallRouter> = {
            let cfg = ToolRouterConfig::from_env();
            match ToolCallRouter::from_config(&cfg) {
                Ok(r) => r,
                Err(e) => {
                    tracing::warn!(error = %e, "FunctionGemma tool router init failed; disabled");
                    None
                }
            }
        };

        for step in 1..=max_steps {
            tracing::info!(step = step, "Agent step starting");
            let _ = event_tx.send(SessionEvent::Thinking).await;

            // Build messages with system prompt first
            let mut messages = vec![Message {
                role: Role::System,
                content: vec![ContentPart::Text {
                    text: system_prompt.clone(),
                }],
            }];
            messages.extend(self.messages.clone());

            let request = CompletionRequest {
                messages,
                tools: tool_definitions.clone(),
                model: model.clone(),
                temperature,
                top_p: None,
                max_tokens: Some(8192),
                stop: Vec::new(),
            };

            let response = provider.complete(request).await?;

            // Optionally route text-only responses through FunctionGemma to
            // produce structured tool calls.  Skipped for native tool-calling models.
            #[cfg(feature = "functiongemma")]
            let response = if let Some(ref router) = tool_router {
                router
                    .maybe_reformat(response, &tool_definitions, model_supports_tools)
                    .await
            } else {
                response
            };

            crate::telemetry::TOKEN_USAGE.record_model_usage(
                &model,
                response.usage.prompt_tokens as u64,
                response.usage.completion_tokens as u64,
            );

            // Extract tool calls
            let tool_calls: Vec<(String, String, serde_json::Value)> = response
                .message
                .content
                .iter()
                .filter_map(|part| {
                    if let ContentPart::ToolCall {
                        id,
                        name,
                        arguments,
                    } = part
                    {
                        let args: serde_json::Value =
                            serde_json::from_str(arguments).unwrap_or(serde_json::json!({}));
                        Some((id.clone(), name.clone(), args))
                    } else {
                        None
                    }
                })
                .collect();

            // Collect text output
            for part in &response.message.content {
                if let ContentPart::Text { text } = part {
                    if !text.is_empty() {
                        final_output.push_str(text);
                        final_output.push('\n');
                        let _ = event_tx.send(SessionEvent::TextChunk(text.clone())).await;
                    }
                }
            }

            if tool_calls.is_empty() {
                self.add_message(response.message.clone());
                break;
            }

            self.add_message(response.message.clone());

            tracing::info!(
                step = step,
                num_tools = tool_calls.len(),
                "Executing tool calls"
            );

            // Execute each tool call with events
            for (tool_id, tool_name, tool_input) in tool_calls {
                let args_str = serde_json::to_string(&tool_input).unwrap_or_default();
                let _ = event_tx
                    .send(SessionEvent::ToolCallStart {
                        name: tool_name.clone(),
                        arguments: args_str,
                    })
                    .await;

                tracing::info!(tool = %tool_name, tool_id = %tool_id, "Executing tool");

                if is_interactive_tool(&tool_name) {
                    tracing::warn!(tool = %tool_name, "Blocking interactive tool in session loop");
                    let content = "Error: Interactive tool 'question' is disabled in this interface. Ask the user directly in assistant text.".to_string();
                    let _ = event_tx
                        .send(SessionEvent::ToolCallComplete {
                            name: tool_name.clone(),
                            output: content.clone(),
                            success: false,
                        })
                        .await;
                    self.add_message(Message {
                        role: Role::Tool,
                        content: vec![ContentPart::ToolResult {
                            tool_call_id: tool_id,
                            content,
                        }],
                    });
                    continue;
                }

                let (content, success) = if let Some(tool) = tool_registry.get(&tool_name) {
                    match tool.execute(tool_input.clone()).await {
                        Ok(result) => {
                            tracing::info!(tool = %tool_name, success = result.success, "Tool execution completed");
                            (result.output, result.success)
                        }
                        Err(e) => {
                            tracing::warn!(tool = %tool_name, error = %e, "Tool execution failed");
                            (format!("Error: {}", e), false)
                        }
                    }
                } else {
                    tracing::warn!(tool = %tool_name, "Tool not found");
                    (format!("Error: Unknown tool '{}'", tool_name), false)
                };

                let _ = event_tx
                    .send(SessionEvent::ToolCallComplete {
                        name: tool_name.clone(),
                        output: content.clone(),
                        success,
                    })
                    .await;

                self.add_message(Message {
                    role: Role::Tool,
                    content: vec![ContentPart::ToolResult {
                        tool_call_id: tool_id,
                        content,
                    }],
                });
            }
        }

        self.save().await?;

        let _ = event_tx
            .send(SessionEvent::TextComplete(final_output.trim().to_string()))
            .await;
        let _ = event_tx.send(SessionEvent::Done).await;

        Ok(SessionResult {
            text: final_output.trim().to_string(),
            session_id: self.id.clone(),
        })
    }

    /// Generate a title for the session based on the first message
    /// Only sets title if not already set (for initial title generation)
    pub async fn generate_title(&mut self) -> Result<()> {
        if self.title.is_some() {
            return Ok(());
        }

        // Get first user message
        let first_message = self
            .messages
            .iter()
            .find(|m| m.role == crate::provider::Role::User);

        if let Some(msg) = first_message {
            let text: String = msg
                .content
                .iter()
                .filter_map(|p| match p {
                    crate::provider::ContentPart::Text { text } => Some(text.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join(" ");

            // Truncate to reasonable length
            self.title = Some(truncate_with_ellipsis(&text, 47));
        }

        Ok(())
    }

    /// Regenerate the title based on the first message, even if already set
    /// Use this for on-demand title updates or after context changes
    pub async fn regenerate_title(&mut self) -> Result<()> {
        // Get first user message
        let first_message = self
            .messages
            .iter()
            .find(|m| m.role == crate::provider::Role::User);

        if let Some(msg) = first_message {
            let text: String = msg
                .content
                .iter()
                .filter_map(|p| match p {
                    crate::provider::ContentPart::Text { text } => Some(text.clone()),
                    _ => None,
                })
                .collect::<Vec<_>>()
                .join(" ");

            // Truncate to reasonable length
            self.title = Some(truncate_with_ellipsis(&text, 47));
        }

        Ok(())
    }

    /// Set a custom title for the session
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = Some(title.into());
        self.updated_at = Utc::now();
    }

    /// Clear the title, allowing it to be regenerated
    pub fn clear_title(&mut self) {
        self.title = None;
        self.updated_at = Utc::now();
    }

    /// Handle context change - updates metadata and optionally regenerates title
    /// Call this when the session context changes (e.g., directory change, model change)
    pub async fn on_context_change(&mut self, regenerate_title: bool) -> Result<()> {
        self.updated_at = Utc::now();

        if regenerate_title {
            self.regenerate_title().await?;
        }

        Ok(())
    }

    /// Get the sessions directory
    fn sessions_dir() -> Result<PathBuf> {
        crate::config::Config::data_dir()
            .map(|d| d.join("sessions"))
            .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))
    }

    /// Get the path for a session file
    fn session_path(id: &str) -> Result<PathBuf> {
        Ok(Self::sessions_dir()?.join(format!("{}.json", id)))
    }
}

/// Result from a session prompt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionResult {
    pub text: String,
    pub session_id: String,
}

/// Events emitted during session processing for real-time UI updates
#[derive(Debug, Clone)]
pub enum SessionEvent {
    /// Agent is thinking/processing
    Thinking,
    /// Tool call started
    ToolCallStart { name: String, arguments: String },
    /// Tool call completed with result
    ToolCallComplete {
        name: String,
        output: String,
        success: bool,
    },
    /// Partial text output (for streaming)
    TextChunk(String),
    /// Final text output
    TextComplete(String),
    /// Processing complete
    Done,
    /// Error occurred
    Error(String),
}

/// List all sessions
pub async fn list_sessions() -> Result<Vec<SessionSummary>> {
    let sessions_dir = crate::config::Config::data_dir()
        .map(|d| d.join("sessions"))
        .ok_or_else(|| anyhow::anyhow!("Could not determine data directory"))?;

    if !sessions_dir.exists() {
        return Ok(Vec::new());
    }

    let mut summaries = Vec::new();
    let mut entries = fs::read_dir(&sessions_dir).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.extension().map(|e| e == "json").unwrap_or(false) {
            if let Ok(content) = fs::read_to_string(&path).await {
                if let Ok(session) = serde_json::from_str::<Session>(&content) {
                    summaries.push(SessionSummary {
                        id: session.id,
                        title: session.title,
                        created_at: session.created_at,
                        updated_at: session.updated_at,
                        message_count: session.messages.len(),
                        agent: session.agent,
                        directory: session.metadata.directory,
                    });
                }
            }
        }
    }

    summaries.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(summaries)
}

/// List sessions scoped to a specific directory (workspace)
///
/// Only returns sessions whose `metadata.directory` matches the given path.
/// This prevents sessions from other workspaces "leaking" into the TUI.
pub async fn list_sessions_for_directory(dir: &std::path::Path) -> Result<Vec<SessionSummary>> {
    let all = list_sessions().await?;
    let canonical = dir.canonicalize().unwrap_or_else(|_| dir.to_path_buf());
    Ok(all
        .into_iter()
        .filter(|s| {
            s.directory
                .as_ref()
                .map(|d| d.canonicalize().unwrap_or_else(|_| d.clone()) == canonical)
                .unwrap_or(false)
        })
        .collect())
}

/// Summary of a session for listing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
    pub id: String,
    pub title: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub message_count: usize,
    pub agent: String,
    /// The working directory this session was created in
    #[serde(default)]
    pub directory: Option<PathBuf>,
}

fn truncate_with_ellipsis(value: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }

    let mut chars = value.chars();
    let mut output = String::new();
    for _ in 0..max_chars {
        if let Some(ch) = chars.next() {
            output.push(ch);
        } else {
            return value.to_string();
        }
    }

    if chars.next().is_some() {
        format!("{output}...")
    } else {
        output
    }
}

// Async helper for Vec - kept for potential future use
#[allow(dead_code)]
use futures::StreamExt;

#[allow(dead_code)]
trait AsyncCollect<T> {
    async fn collect(self) -> Vec<T>;
}

#[allow(dead_code)]
impl<S, T> AsyncCollect<T> for S
where
    S: futures::Stream<Item = T> + Unpin,
{
    async fn collect(mut self) -> Vec<T> {
        let mut items = Vec::new();
        while let Some(item) = self.next().await {
            items.push(item);
        }
        items
    }
}
