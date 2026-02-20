//! Session management
//!
//! Sessions track the conversation history and state for agent interactions.

use crate::agent::ToolUse;
use crate::audit::{AuditCategory, AuditOutcome, try_audit_log};
use crate::event_stream::ChatEvent;
use crate::event_stream::s3_sink::S3Sink;
use crate::provider::{Message, Usage};
use crate::rlm::router::AutoProcessContext;
use crate::rlm::{RlmChunker, RlmConfig, RlmRouter, RoutingContext};
use crate::tool::ToolRegistry;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs;
use uuid::Uuid;

use crate::cognition::tool_router::{ToolCallRouter, ToolRouterConfig};

fn is_interactive_tool(tool_name: &str) -> bool {
    matches!(tool_name, "question")
}

fn choose_default_provider<'a>(providers: &'a [&'a str]) -> Option<&'a str> {
    // Keep Google as an explicit option, but don't default to it first because
    // some environments expose API keys that are not valid for ChatCompletions.
    let preferred = [
        "zai",
        "openai",
        "github-copilot",
        "anthropic",
        "minimax",
        "openrouter",
        "novita",
        "moonshotai",
        "google",
    ];
    for name in preferred {
        if let Some(found) = providers.iter().copied().find(|p| *p == name) {
            return Some(found);
        }
    }
    providers.first().copied()
}

fn prefers_temperature_one(model: &str) -> bool {
    let normalized = model.to_ascii_lowercase();
    normalized.contains("kimi-k2") || normalized.contains("glm-") || normalized.contains("minimax")
}

/// Return the context window size (in tokens) for known models.
fn context_window_for_model(model: &str) -> usize {
    let m = model.to_ascii_lowercase();
    if m.contains("kimi-k2") {
        256_000
    } else if m.contains("glm-5") || m.contains("glm5") {
        200_000
    } else if m.contains("gpt-4o") {
        128_000
    } else if m.contains("gpt-5") {
        256_000
    } else if m.contains("claude") {
        200_000
    } else if m.contains("gemini") {
        1_000_000
    } else if m.contains("minimax") || m.contains("m2.5") {
        256_000
    } else if m.contains("qwen") {
        131_072
    } else {
        128_000 // conservative default
    }
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
    /// Optional bus for publishing agent thinking/reasoning to training pipeline
    #[serde(skip)]
    pub bus: Option<Arc<crate::bus::AgentBus>>,
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
            "minimax" => "MiniMax-M2.5".to_string(),
            "openai" => "gpt-4o".to_string(),
            "google" => "gemini-2.5-pro".to_string(),
            "zhipuai" | "zai" => "glm-5".to_string(),
            // OpenRouter uses model IDs like "z-ai/glm-5".
            "openrouter" => "z-ai/glm-5".to_string(),
            "novita" => "qwen/qwen3-coder-next".to_string(),
            "github-copilot" | "github-copilot-enterprise" => "gpt-5-mini".to_string(),
            _ => "glm-5".to_string(),
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
            bus: None,
        })
    }

    /// Attach a bus for publishing agent thinking/reasoning
    pub fn with_bus(mut self, bus: Arc<crate::bus::AgentBus>) -> Self {
        self.bus = Some(bus);
        self
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

        let canonical_workspace =
            workspace.map(|w| w.canonicalize().unwrap_or_else(|_| w.to_path_buf()));

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
            let prov = prov.map(|p| if p == "zhipuai" { "zai" } else { p });
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

        // Determine which provider to use with deterministic fallback ordering.
        let selected_provider = provider_name
            .as_deref()
            .filter(|p| providers.contains(p))
            .or_else(|| choose_default_provider(providers.as_slice()))
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

        // Compress oversized user message via RLM if it exceeds the context threshold
        {
            let ctx_window = context_window_for_model(&model);
            let msg_tokens = RlmChunker::estimate_tokens(message);
            let threshold = (ctx_window as f64 * 0.35) as usize;
            if msg_tokens > threshold {
                tracing::info!(
                    msg_tokens,
                    threshold,
                    ctx_window,
                    "RLM: User message exceeds context threshold, compressing"
                );
                let auto_ctx = AutoProcessContext {
                    tool_id: "session_context",
                    tool_args: serde_json::json!({}),
                    session_id: &self.id,
                    abort: None,
                    on_progress: None,
                    provider: Arc::clone(&provider),
                    model: model.clone(),
                };
                let rlm_config = RlmConfig::default();
                match RlmRouter::auto_process(message, auto_ctx, &rlm_config).await {
                    Ok(result) => {
                        tracing::info!(
                            input_tokens = result.stats.input_tokens,
                            output_tokens = result.stats.output_tokens,
                            "RLM: User message compressed"
                        );
                        // Replace the last message (user message we just added)
                        if let Some(last) = self.messages.last_mut() {
                            last.content = vec![ContentPart::Text {
                                text: format!(
                                    "[Original message: {} tokens, compressed via RLM]\n\n{}\n\n---\nOriginal request prefix:\n{}",
                                    msg_tokens,
                                    result.processed,
                                    message.chars().take(500).collect::<String>()
                                ),
                            }];
                        }
                    }
                    Err(e) => {
                        tracing::warn!(error = %e, "RLM: Failed to compress user message, using truncation");
                        let max_chars = threshold * 4;
                        let truncated = RlmChunker::compress(message, max_chars / 4, None);
                        if let Some(last) = self.messages.last_mut() {
                            last.content = vec![ContentPart::Text { text: truncated }];
                        }
                    }
                }
            }
        }

        // Create tool registry with all available tools
        let tool_registry = ToolRegistry::with_provider_arc(Arc::clone(&provider), model.clone());
        let tool_definitions: Vec<_> = tool_registry
            .definitions()
            .into_iter()
            .filter(|tool| !is_interactive_tool(&tool.name))
            .collect();

        // Some models behave best with temperature=1.0.
        // - Kimi K2.x requires temperature=1.0
        // - GLM (Z.AI) defaults to temperature 1.0 for coding workflows
        // Use contains() to match both short aliases and provider-qualified IDs.
        let temperature = if prefers_temperature_one(&model) {
            Some(1.0)
        } else {
            Some(0.7)
        };

        tracing::info!("Using model: {} via provider: {}", model, selected_provider);
        tracing::info!("Available tools: {}", tool_definitions.len());

        // All current providers support native tool calling.  Hardcode to
        // true so we skip the expensive list_models() API call on every message.

        let model_supports_tools = true;

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
                    .. } = part
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

            // Collect text output and publish thinking to bus
            for part in &response.message.content {
                match part {
                    ContentPart::Text { text } if !text.is_empty() => {
                        final_output.push_str(text);
                        final_output.push('\n');
                    }
                    ContentPart::Thinking { text } if !text.is_empty() => {
                        if let Some(ref bus) = self.bus {
                            let handle = bus.handle(&self.agent);
                            handle.send(
                                format!("agent.{}.thinking", self.agent),
                                crate::bus::BusMessage::AgentThinking {
                                    agent_id: self.agent.clone(),
                                    thinking: text.clone(),
                                    step,
                                },
                            );
                        }
                    }
                    _ => {}
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

                // Publish tool request to bus for training pipeline
                if let Some(ref bus) = self.bus {
                    let handle = bus.handle(&self.agent);
                    handle.send(
                        format!("agent.{}.tool.request", self.agent),
                        crate::bus::BusMessage::ToolRequest {
                            request_id: tool_id.clone(),
                            agent_id: self.agent.clone(),
                            tool_name: tool_name.clone(),
                            arguments: tool_input.clone(),
                        },
                    );
                }

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
                let exec_start = std::time::Instant::now();
                let content = if let Some(tool) = tool_registry.get(&tool_name) {
                    match tool.execute(tool_input.clone()).await {
                        Ok(result) => {
                            let duration_ms = exec_start.elapsed().as_millis() as u64;
                            tracing::info!(tool = %tool_name, success = result.success, "Tool execution completed");
                            if let Some(audit) = try_audit_log() {
                                audit.log_with_correlation(
                                        AuditCategory::ToolExecution,
                                        format!("tool:{}", tool_name),
                                        if result.success { AuditOutcome::Success } else { AuditOutcome::Failure },
                                        None,
                                        Some(json!({ "duration_ms": duration_ms, "output_len": result.output.len() })),
                                        None,  // okr_id
                                        None,  // okr_run_id
                                        None,  // relay_id
                                        Some(self.id.clone()),  // session_id
                                    ).await;
                            }
                            result.output
                        }
                        Err(e) => {
                            let duration_ms = exec_start.elapsed().as_millis() as u64;
                            tracing::warn!(tool = %tool_name, error = %e, "Tool execution failed");
                            if let Some(audit) = try_audit_log() {
                                audit.log_with_correlation(
                                        AuditCategory::ToolExecution,
                                        format!("tool:{}", tool_name),
                                        AuditOutcome::Failure,
                                        None,
                                        Some(json!({ "duration_ms": duration_ms, "error": e.to_string() })),
                                        None,  // okr_id
                                        None,  // okr_run_id
                                        None,  // relay_id
                                        Some(self.id.clone()),  // session_id
                                    ).await;
                            }
                            format!("Error: {}", e)
                        }
                    }
                } else {
                    tracing::warn!(tool = %tool_name, "Tool not found");
                    if let Some(audit) = try_audit_log() {
                        audit
                            .log_with_correlation(
                                AuditCategory::ToolExecution,
                                format!("tool:{}", tool_name),
                                AuditOutcome::Failure,
                                None,
                                Some(json!({ "error": "unknown_tool" })),
                                None,                  // okr_id
                                None,                  // okr_run_id
                                None,                  // relay_id
                                Some(self.id.clone()), // session_id
                            )
                            .await;
                    }
                    format!("Error: Unknown tool '{}'", tool_name)
                };

                // Calculate duration for event stream
                let duration_ms = exec_start.elapsed().as_millis() as u64;
                let success = !content.starts_with("Error:");

                // Publish full tool output to bus for training pipeline
                // (before RLM truncation so we capture the complete output)
                if let Some(ref bus) = self.bus {
                    let handle = bus.handle(&self.agent);
                    handle.send(
                        format!("agent.{}.tool.output", self.agent),
                        crate::bus::BusMessage::ToolOutputFull {
                            agent_id: self.agent.clone(),
                            tool_name: tool_name.clone(),
                            output: content.clone(),
                            success,
                            step,
                        },
                    );
                }

                // Emit event stream event for audit/compliance (SOC 2, FedRAMP, ATO)
                if let Some(base_dir) = Self::event_stream_path() {
                    let workspace = std::env::var("PWD")
                        .map(PathBuf::from)
                        .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default());
                    let event = ChatEvent::tool_result(
                        workspace,
                        self.id.clone(),
                        &tool_name,
                        success,
                        duration_ms,
                        &content,
                        self.messages.len() as u64,
                    );
                    let event_json = event.to_json();
                    let timestamp = Utc::now().format("%Y%m%dT%H%M%SZ");
                    let seq = self.messages.len() as u64;
                    let filename = format!(
                        "{}-chat-events-{:020}-{:020}.jsonl",
                        timestamp,
                        seq * 10000,
                        (seq + 1) * 10000
                    );
                    let event_path = base_dir.join(&self.id).join(filename);

                    let event_path_clone = event_path;
                    tokio::spawn(async move {
                        if let Some(parent) = event_path_clone.parent() {
                            let _ = tokio::fs::create_dir_all(parent).await;
                        }
                        if let Ok(mut file) = tokio::fs::OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(&event_path_clone)
                            .await
                        {
                            use tokio::io::AsyncWriteExt;
                            let _ = file.write_all(event_json.as_bytes()).await;
                            let _ = file.write_all(b"\n").await;
                        }
                    });
                }

                // Route large tool outputs through RLM
                let content = {
                    let ctx_window = context_window_for_model(&model);
                    let total_chars: usize = self
                        .messages
                        .iter()
                        .map(|m| {
                            m.content
                                .iter()
                                .map(|p| match p {
                                    ContentPart::Text { text } => text.len(),
                                    ContentPart::ToolResult { content, .. } => content.len(),
                                    _ => 0,
                                })
                                .sum::<usize>()
                        })
                        .sum();
                    let current_tokens = total_chars / 4; // ~4 chars per token
                    let routing_ctx = RoutingContext {
                        tool_id: tool_name.clone(),
                        session_id: self.id.clone(),
                        call_id: Some(tool_id.clone()),
                        model_context_limit: ctx_window,
                        current_context_tokens: Some(current_tokens),
                    };
                    let rlm_config = RlmConfig::default();
                    let routing = RlmRouter::should_route(&content, &routing_ctx, &rlm_config);
                    if routing.should_route {
                        tracing::info!(
                            tool = %tool_name,
                            reason = %routing.reason,
                            estimated_tokens = routing.estimated_tokens,
                            "RLM: Routing large tool output"
                        );
                        let auto_ctx = AutoProcessContext {
                            tool_id: &tool_name,
                            tool_args: tool_input.clone(),
                            session_id: &self.id,
                            abort: None,
                            on_progress: None,
                            provider: Arc::clone(&provider),
                            model: model.clone(),
                        };
                        match RlmRouter::auto_process(&content, auto_ctx, &rlm_config).await {
                            Ok(result) => {
                                tracing::info!(
                                    input_tokens = result.stats.input_tokens,
                                    output_tokens = result.stats.output_tokens,
                                    iterations = result.stats.iterations,
                                    "RLM: Processing complete"
                                );
                                result.processed
                            }
                            Err(e) => {
                                tracing::warn!(error = %e, "RLM: auto_process failed, using smart_truncate");
                                let (truncated, _, _) = RlmRouter::smart_truncate(
                                    &content,
                                    &tool_name,
                                    &tool_input,
                                    ctx_window / 4,
                                );
                                truncated
                            }
                        }
                    } else {
                        content
                    }
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

        // Archive event stream to S3/R2 if configured (for compliance: SOC 2, FedRAMP, ATO)
        self.archive_event_stream_to_s3().await;

        Ok(SessionResult {
            text: final_output.trim().to_string(),
            session_id: self.id.clone(),
        })
    }

    /// Archive event stream files to S3/R2 for immutable compliance logging
    async fn archive_event_stream_to_s3(&self) {
        // Check if S3 is configured
        if !S3Sink::is_configured() {
            return;
        }

        let Some(base_dir) = Self::event_stream_path() else {
            return;
        };

        let session_event_dir = base_dir.join(&self.id);
        if !session_event_dir.exists() {
            return;
        }

        // Try to create S3 sink
        let Ok(sink) = S3Sink::from_env().await else {
            tracing::warn!("Failed to create S3 sink for archival");
            return;
        };

        // Upload all event files in the session directory
        let session_id = self.id.clone();
        tokio::spawn(async move {
            if let Ok(mut entries) = tokio::fs::read_dir(&session_event_dir).await {
                while let Ok(Some(entry)) = entries.next_entry().await {
                    let path = entry.path();
                    if path.extension().map(|e| e == "jsonl").unwrap_or(false) {
                        match sink.upload_file(&path, &session_id).await {
                            Ok(url) => {
                                tracing::info!(url = %url, "Archived event stream to S3/R2");
                            }
                            Err(e) => {
                                tracing::warn!(error = %e, "Failed to archive event file to S3");
                            }
                        }
                    }
                }
            }
        });
    }

    /// Process a user message with real-time event streaming for UI updates.
    /// Events are sent through the provided channel as tool calls execute.
    ///
    /// Accepts a pre-loaded `ProviderRegistry` to avoid re-fetching secrets
    /// from Vault on every message (which was the primary TUI performance
    /// bottleneck).
    pub async fn prompt_with_events(
        &mut self,
        message: &str,
        event_tx: tokio::sync::mpsc::Sender<SessionEvent>,
        registry: std::sync::Arc<crate::provider::ProviderRegistry>,
    ) -> Result<SessionResult> {
        use crate::provider::{CompletionRequest, ContentPart, Role, parse_model_string};

        let _ = event_tx.send(SessionEvent::Thinking).await;

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
            let prov = prov.map(|p| if p == "zhipuai" { "zai" } else { p });
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

        // Determine which provider to use with deterministic fallback ordering.
        let selected_provider = provider_name
            .as_deref()
            .filter(|p| providers.contains(p))
            .or_else(|| choose_default_provider(providers.as_slice()))
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

        // Compress oversized user message via RLM if it exceeds the context threshold
        {
            let ctx_window = context_window_for_model(&model);
            let msg_tokens = RlmChunker::estimate_tokens(message);
            let threshold = (ctx_window as f64 * 0.35) as usize;
            if msg_tokens > threshold {
                tracing::info!(
                    msg_tokens,
                    threshold,
                    ctx_window,
                    "RLM: User message exceeds context threshold, compressing"
                );
                let auto_ctx = AutoProcessContext {
                    tool_id: "session_context",
                    tool_args: serde_json::json!({}),
                    session_id: &self.id,
                    abort: None,
                    on_progress: None,
                    provider: Arc::clone(&provider),
                    model: model.clone(),
                };
                let rlm_config = RlmConfig::default();
                match RlmRouter::auto_process(message, auto_ctx, &rlm_config).await {
                    Ok(result) => {
                        tracing::info!(
                            input_tokens = result.stats.input_tokens,
                            output_tokens = result.stats.output_tokens,
                            "RLM: User message compressed"
                        );
                        if let Some(last) = self.messages.last_mut() {
                            last.content = vec![ContentPart::Text {
                                text: format!(
                                    "[Original message: {} tokens, compressed via RLM]\n\n{}\n\n---\nOriginal request prefix:\n{}",
                                    msg_tokens,
                                    result.processed,
                                    message.chars().take(500).collect::<String>()
                                ),
                            }];
                        }
                    }
                    Err(e) => {
                        tracing::warn!(error = %e, "RLM: Failed to compress user message, using truncation");
                        let max_chars = threshold * 4;
                        let truncated = RlmChunker::compress(message, max_chars / 4, None);
                        if let Some(last) = self.messages.last_mut() {
                            last.content = vec![ContentPart::Text { text: truncated }];
                        }
                    }
                }
            }
        }

        // Create tool registry
        let tool_registry = ToolRegistry::with_provider_arc(Arc::clone(&provider), model.clone());
        let tool_definitions: Vec<_> = tool_registry
            .definitions()
            .into_iter()
            .filter(|tool| !is_interactive_tool(&tool.name))
            .collect();

        let temperature = if prefers_temperature_one(&model) {
            Some(1.0)
        } else {
            Some(0.7)
        };

        tracing::info!("Using model: {} via provider: {}", model, selected_provider);
        tracing::info!("Available tools: {}", tool_definitions.len());

        // All current providers support native tool calling.  Hardcode to
        // true so we skip the expensive list_models() API call on every message.

        let model_supports_tools = true;

        // Build system prompt
        let cwd = std::env::var("PWD")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default());
        let system_prompt = crate::agent::builtin::build_system_prompt(&cwd);

        let mut final_output = String::new();
        let max_steps = 50;

        // Initialise the FunctionGemma tool-call router (feature-gated, opt-in).

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

            let llm_start = std::time::Instant::now();
            let response = provider.complete(request).await?;
            let llm_duration_ms = llm_start.elapsed().as_millis() as u64;

            // Optionally route text-only responses through FunctionGemma to
            // produce structured tool calls.  Skipped for native tool-calling models.

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

            // Emit usage report for TUI display
            let _ = event_tx
                .send(SessionEvent::UsageReport {
                    prompt_tokens: response.usage.prompt_tokens,
                    completion_tokens: response.usage.completion_tokens,
                    duration_ms: llm_duration_ms,
                    model: model.clone(),
                })
                .await;

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
                    .. } = part
                    {
                        let args: serde_json::Value =
                            serde_json::from_str(arguments).unwrap_or(serde_json::json!({}));
                        Some((id.clone(), name.clone(), args))
                    } else {
                        None
                    }
                })
                .collect();

            // Collect text output for this step
            // Collect thinking and text output
            let mut thinking_text = String::new();
            let mut step_text = String::new();
            for part in &response.message.content {
                match part {
                    ContentPart::Thinking { text } => {
                        if !text.is_empty() {
                            thinking_text.push_str(text);
                            thinking_text.push('\n');
                        }
                    }
                    ContentPart::Text { text } => {
                        if !text.is_empty() {
                            step_text.push_str(text);
                            step_text.push('\n');
                        }
                    }
                    _ => {}
                }
            }

            // Emit thinking output first
            if !thinking_text.trim().is_empty() {
                let _ = event_tx
                    .send(SessionEvent::ThinkingComplete(
                        thinking_text.trim().to_string(),
                    ))
                    .await;
                if let Some(ref bus) = self.bus {
                    let handle = bus.handle(&self.agent);
                    handle.send(
                        format!("agent.{}.thinking", self.agent),
                        crate::bus::BusMessage::AgentThinking {
                            agent_id: self.agent.clone(),
                            thinking: thinking_text.trim().to_string(),
                            step,
                        },
                    );
                }
            }

            // Emit this step's text BEFORE tool calls so it appears in correct
            // chronological order in the TUI chat display.
            if !step_text.trim().is_empty() {
                let trimmed = step_text.trim().to_string();
                let _ = event_tx
                    .send(SessionEvent::TextChunk(trimmed.clone()))
                    .await;
                let _ = event_tx.send(SessionEvent::TextComplete(trimmed)).await;
                final_output.push_str(&step_text);
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

                // Publish tool request to bus for training pipeline
                if let Some(ref bus) = self.bus {
                    let handle = bus.handle(&self.agent);
                    handle.send(
                        format!("agent.{}.tool.request", self.agent),
                        crate::bus::BusMessage::ToolRequest {
                            request_id: tool_id.clone(),
                            agent_id: self.agent.clone(),
                            tool_name: tool_name.clone(),
                            arguments: tool_input.clone(),
                        },
                    );
                }

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

                let exec_start = std::time::Instant::now();
                let (content, success) = if let Some(tool) = tool_registry.get(&tool_name) {
                    match tool.execute(tool_input.clone()).await {
                        Ok(result) => {
                            let duration_ms = exec_start.elapsed().as_millis() as u64;
                            tracing::info!(tool = %tool_name, success = result.success, "Tool execution completed");
                            if let Some(audit) = try_audit_log() {
                                audit.log_with_correlation(
                                        AuditCategory::ToolExecution,
                                        format!("tool:{}", tool_name),
                                        if result.success { AuditOutcome::Success } else { AuditOutcome::Failure },
                                        None,
                                        Some(json!({ "duration_ms": duration_ms, "output_len": result.output.len() })),
                                        None,  // okr_id
                                        None,  // okr_run_id
                                        None,  // relay_id
                                        Some(self.id.clone()),  // session_id
                                    ).await;
                            }
                            (result.output, result.success)
                        }
                        Err(e) => {
                            let duration_ms = exec_start.elapsed().as_millis() as u64;
                            tracing::warn!(tool = %tool_name, error = %e, "Tool execution failed");
                            if let Some(audit) = try_audit_log() {
                                audit.log_with_correlation(
                                        AuditCategory::ToolExecution,
                                        format!("tool:{}", tool_name),
                                        AuditOutcome::Failure,
                                        None,
                                        Some(json!({ "duration_ms": duration_ms, "error": e.to_string() })),
                                        None,  // okr_id
                                        None,  // okr_run_id
                                        None,  // relay_id
                                        Some(self.id.clone()),  // session_id
                                    ).await;
                            }
                            (format!("Error: {}", e), false)
                        }
                    }
                } else {
                    tracing::warn!(tool = %tool_name, "Tool not found");
                    if let Some(audit) = try_audit_log() {
                        audit
                            .log_with_correlation(
                                AuditCategory::ToolExecution,
                                format!("tool:{}", tool_name),
                                AuditOutcome::Failure,
                                None,
                                Some(json!({ "error": "unknown_tool" })),
                                None,                  // okr_id
                                None,                  // okr_run_id
                                None,                  // relay_id
                                Some(self.id.clone()), // session_id
                            )
                            .await;
                    }
                    (format!("Error: Unknown tool '{}'", tool_name), false)
                };

                // Calculate total duration from exec_start (captured from line 772)
                let duration_ms = exec_start.elapsed().as_millis() as u64;

                // Publish full tool output to bus for training pipeline
                if let Some(ref bus) = self.bus {
                    let handle = bus.handle(&self.agent);
                    handle.send(
                        format!("agent.{}.tool.output", self.agent),
                        crate::bus::BusMessage::ToolOutputFull {
                            agent_id: self.agent.clone(),
                            tool_name: tool_name.clone(),
                            output: content.clone(),
                            success,
                            step,
                        },
                    );
                }

                // Emit event stream event for audit/compliance (SOC 2, FedRAMP, ATO)
                // This creates the structured JSONL record with byte-range offsets
                // File format: {timestamp}-chat-events-{start_byte}-{end_byte}.jsonl
                if let Some(base_dir) = Self::event_stream_path() {
                    let workspace = std::env::var("PWD")
                        .map(PathBuf::from)
                        .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default());
                    let event = ChatEvent::tool_result(
                        workspace,
                        self.id.clone(),
                        &tool_name,
                        success,
                        duration_ms,
                        &content,
                        self.messages.len() as u64,
                    );
                    let event_json = event.to_json();
                    let event_size = event_json.len() as u64 + 1; // +1 for newline

                    // Generate filename with byte-range offsets for random access replay
                    // Format: {timestamp}-chat-events-{start_offset}-{end_offset}.jsonl
                    // We use a session-scoped counter stored in metadata for byte tracking
                    let timestamp = Utc::now().format("%Y%m%dT%H%M%SZ");
                    let seq = self.messages.len() as u64;
                    let filename = format!(
                        "{}-chat-events-{:020}-{:020}.jsonl",
                        timestamp,
                        seq * 10000,       // approximated start offset
                        (seq + 1) * 10000  // approximated end offset
                    );
                    let event_path = base_dir.join(&self.id).join(filename);

                    // Fire-and-forget: don't block tool execution on event logging
                    let event_path_clone = event_path;
                    tokio::spawn(async move {
                        if let Some(parent) = event_path_clone.parent() {
                            let _ = tokio::fs::create_dir_all(parent).await;
                        }
                        if let Ok(mut file) = tokio::fs::OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(&event_path_clone)
                            .await
                        {
                            use tokio::io::AsyncWriteExt;
                            let _ = file.write_all(event_json.as_bytes()).await;
                            let _ = file.write_all(b"\n").await;
                            tracing::debug!(path = %event_path_clone.display(), size = event_size, "Event stream wrote");
                        }
                    });
                }

                let _ = event_tx
                    .send(SessionEvent::ToolCallComplete {
                        name: tool_name.clone(),
                        output: content.clone(),
                        success,
                    })
                    .await;

                // Route large tool outputs through RLM
                let content = {
                    let ctx_window = context_window_for_model(&model);
                    let total_chars: usize = self
                        .messages
                        .iter()
                        .map(|m| {
                            m.content
                                .iter()
                                .map(|p| match p {
                                    ContentPart::Text { text } => text.len(),
                                    ContentPart::ToolResult { content, .. } => content.len(),
                                    _ => 0,
                                })
                                .sum::<usize>()
                        })
                        .sum();
                    let current_tokens = total_chars / 4;
                    let routing_ctx = RoutingContext {
                        tool_id: tool_name.clone(),
                        session_id: self.id.clone(),
                        call_id: Some(tool_id.clone()),
                        model_context_limit: ctx_window,
                        current_context_tokens: Some(current_tokens),
                    };
                    let rlm_config = RlmConfig::default();
                    let routing = RlmRouter::should_route(&content, &routing_ctx, &rlm_config);
                    if routing.should_route {
                        tracing::info!(
                            tool = %tool_name,
                            reason = %routing.reason,
                            estimated_tokens = routing.estimated_tokens,
                            "RLM: Routing large tool output"
                        );
                        let auto_ctx = AutoProcessContext {
                            tool_id: &tool_name,
                            tool_args: tool_input.clone(),
                            session_id: &self.id,
                            abort: None,
                            on_progress: None,
                            provider: Arc::clone(&provider),
                            model: model.clone(),
                        };
                        match RlmRouter::auto_process(&content, auto_ctx, &rlm_config).await {
                            Ok(result) => {
                                tracing::info!(
                                    input_tokens = result.stats.input_tokens,
                                    output_tokens = result.stats.output_tokens,
                                    iterations = result.stats.iterations,
                                    "RLM: Processing complete"
                                );
                                result.processed
                            }
                            Err(e) => {
                                tracing::warn!(error = %e, "RLM: auto_process failed, using smart_truncate");
                                let (truncated, _, _) = RlmRouter::smart_truncate(
                                    &content,
                                    &tool_name,
                                    &tool_input,
                                    ctx_window / 4,
                                );
                                truncated
                            }
                        }
                    } else {
                        content
                    }
                };

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

        // Archive event stream to S3/R2 if configured (for compliance: SOC 2, FedRAMP, ATO)
        self.archive_event_stream_to_s3().await;

        // Text was already sent per-step via TextComplete events.
        // Send updated session state so the caller can sync back.
        let _ = event_tx.send(SessionEvent::SessionSync(self.clone())).await;
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

    /// Delete a session by ID
    pub async fn delete(id: &str) -> Result<()> {
        let path = Self::session_path(id)?;
        if path.exists() {
            tokio::fs::remove_file(&path).await?;
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

    /// Get the event stream path from environment, if configured.
    /// Returns None if CODETETHER_EVENT_STREAM_PATH is not set.
    fn event_stream_path() -> Option<PathBuf> {
        std::env::var("CODETETHER_EVENT_STREAM_PATH")
            .ok()
            .map(PathBuf::from)
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
    /// Model thinking/reasoning output
    ThinkingComplete(String),
    /// Token usage for one LLM round-trip
    UsageReport {
        prompt_tokens: usize,
        completion_tokens: usize,
        duration_ms: u64,
        model: String,
    },
    /// Updated session state for caller to sync back
    SessionSync(Session),
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

/// List sessions for a directory with pagination.
///
/// - `limit`: Maximum number of sessions to return (default: 100)
/// - `offset`: Number of sessions to skip (default: 0)
pub async fn list_sessions_paged(
    dir: &std::path::Path,
    limit: usize,
    offset: usize,
) -> Result<Vec<SessionSummary>> {
    let mut sessions = list_sessions_for_directory(dir).await?;
    sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(sessions.into_iter().skip(offset).take(limit).collect())
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
