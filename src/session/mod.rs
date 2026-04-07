//! Session management
//!
//! Sessions track the conversation history and state for agent interactions.

use crate::agent::ToolUse;
use crate::audit::{AuditCategory, AuditOutcome, try_audit_log};
use crate::event_stream::ChatEvent;
use crate::event_stream::s3_sink::S3Sink;
use crate::provenance::{ClaimProvenance, ExecutionProvenance};
use crate::provider::{ContentPart, Message, Role, ToolDefinition, Usage};
use crate::rlm::router::AutoProcessContext;
use crate::rlm::{RlmChunker, RlmConfig, RlmRouter, RoutingContext};
use crate::tool::ToolRegistry;
use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use tokio::fs;
use uuid::Uuid;

use crate::cognition::tool_router::{ToolCallRouter, ToolRouterConfig};
use crate::tool::Tool;
use crate::tool::confirm_edit::ConfirmEditTool;
use crate::tool::confirm_multiedit::ConfirmMultiEditTool;

pub mod codex_import;
pub mod helper;
mod listing;
mod listing_all;
pub use self::codex_import::{
    import_codex_session_by_id, import_codex_session_path, import_codex_sessions_for_directory,
    load_or_import_session,
};
pub use self::listing::{SessionSummary, list_sessions, list_sessions_for_directory};
pub use self::listing_all::{list_all_sessions_for_directory, list_codex_sessions_for_directory};

use self::helper::bootstrap::{
    inject_tool_prompt, list_tools_bootstrap_definition, list_tools_bootstrap_output,
};
use self::helper::build::{
    build_request_requires_tool, is_build_agent, should_force_build_tool_first_retry,
};
use self::helper::edit::{
    build_pending_confirmation_apply_request, detect_stub_in_tool_input,
    normalize_tool_call_for_execution,
};
use self::helper::error::{
    is_prompt_too_long_error, is_retryable_upstream_error, messages_to_rlm_context,
};
use self::helper::markup::normalize_textual_tool_calls;
use self::helper::provider::{
    prefers_temperature_one, resolve_provider_for_session_request,
    should_retry_missing_native_tool_call,
};
use self::helper::router::{build_proactive_lsp_context_message, choose_router_target};
use self::helper::runtime::{
    enrich_tool_input_with_runtime_context, is_codesearch_no_match_output, is_interactive_tool,
    is_local_cuda_provider, local_cuda_light_system_prompt,
};
use self::helper::stream::collect_stream_completion_with_events;
use self::helper::text::{extract_text_content, truncate_with_ellipsis};
use self::helper::token::{
    context_window_for_model, estimate_request_tokens, estimate_tokens_for_messages,
    session_completion_max_tokens,
};
use self::helper::validation::{
    build_validation_report, capture_git_dirty_files, track_touched_files,
};
/// An image attachment to include with a message (from clipboard paste, etc.)
#[derive(Debug, Clone)]
pub struct ImageAttachment {
    /// Base64-encoded data URL (e.g., "data:image/png;base64,...")
    pub data_url: String,
    /// MIME type (e.g., "image/png")
    pub mime_type: Option<String>,
}

const BUILD_MODE_TOOL_FIRST_NUDGE: &str = "Build mode policy reminder: execute directly. \
Start by calling at least one appropriate tool now (or emit <tool_call> markup for non-native \
tool providers). Do not ask for permission and do not provide a plan-only response.";
const BUILD_MODE_TOOL_FIRST_MAX_RETRIES: u8 = 2;
const NATIVE_TOOL_PROMISE_RETRY_MAX_RETRIES: u8 = 1;
const MAX_CONSECUTIVE_CODESEARCH_NO_MATCHES: u32 = 5;
const POST_EDIT_VALIDATION_MAX_RETRIES: u8 = 3;
const CODESEARCH_THRASH_NUDGE: &str = "Stop brute-force codesearch variant retries. \
You already got repeated \"No matches found\" results. Do not try punctuation/casing/underscore \
variants of the same token again. Either switch to a broader strategy (e.g., inspect likely files \
directly) or conclude the identifier is absent and continue with the best available evidence.";
const NATIVE_TOOL_PROMISE_NUDGE: &str = "You said you would use tools. Do not describe the tool \
call or promise a next step. Emit the actual tool call now. If native tool calling fails, emit a \
<tool_call> JSON block immediately instead of prose.";

fn pending_confirmation_tool_result_content(tool_name: &str, content: &str) -> String {
    format!(
        "{content}\n\nStatus: Pending confirmation only. `{tool_name}` has NOT been applied yet. \
         Auto-apply is off. Enable it in TUI Settings or with `/autoapply on` if you want pending \
         edit previews to be confirmed automatically."
    )
}

fn auto_apply_pending_confirmation_result_content(output: &str, success: bool) -> String {
    let status = if success {
        "TUI edit auto-apply is enabled. The pending change was automatically confirmed and applied."
    } else {
        "TUI edit auto-apply is enabled, but confirming the pending change failed."
    };
    format!("{status}\n\n{output}")
}

fn tool_result_requires_confirmation(
    tool_metadata: Option<&HashMap<String, serde_json::Value>>,
) -> bool {
    tool_metadata
        .and_then(|metadata| metadata.get("requires_confirmation"))
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false)
}

async fn auto_apply_pending_confirmation(
    tool_name: &str,
    tool_input: &serde_json::Value,
    tool_metadata: Option<&HashMap<String, serde_json::Value>>,
) -> Result<Option<(String, bool, Option<HashMap<String, serde_json::Value>>)>> {
    let Some((confirm_tool_name, confirm_input)) =
        build_pending_confirmation_apply_request(tool_name, tool_input, tool_metadata)
    else {
        return Ok(None);
    };

    let result = match confirm_tool_name.as_str() {
        "confirm_edit" => ConfirmEditTool::new().execute(confirm_input).await?,
        "confirm_multiedit" => ConfirmMultiEditTool::new().execute(confirm_input).await?,
        _ => return Ok(None),
    };

    let metadata = if result.metadata.is_empty() {
        tool_metadata.cloned()
    } else {
        Some(result.metadata)
    };

    Ok(Some((
        auto_apply_pending_confirmation_result_content(&result.output, result.success),
        result.success,
        metadata,
    )))
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
    pub knowledge_snapshot: Option<PathBuf>,
    pub provenance: Option<ExecutionProvenance>,
    #[serde(default)]
    pub auto_apply_edits: bool,
    #[serde(default)]
    pub allow_network: bool,
    #[serde(default = "default_slash_autocomplete")]
    pub slash_autocomplete: bool,
    #[serde(default = "default_use_worktree")]
    pub use_worktree: bool,
    pub shared: bool,
    pub share_url: Option<String>,
}

fn default_slash_autocomplete() -> bool {
    true
}

fn default_use_worktree() -> bool {
    true
}

impl Session {
    fn default_model_for_provider(provider: &str) -> String {
        match provider {
            "moonshotai" => "kimi-k2.5".to_string(),
            "anthropic" => "claude-sonnet-4-20250514".to_string(),
            "minimax" => "MiniMax-M2.5".to_string(),
            "openai" => "gpt-4o".to_string(),
            "google" => "gemini-2.5-pro".to_string(),
            "local_cuda" => std::env::var("LOCAL_CUDA_MODEL")
                .or_else(|_| std::env::var("CODETETHER_LOCAL_CUDA_MODEL"))
                .unwrap_or_else(|_| "qwen3.5-9b".to_string()),
            "zhipuai" | "zai" | "zai-api" => "glm-5".to_string(),
            // OpenRouter uses model IDs like "z-ai/glm-5".
            "openrouter" => "z-ai/glm-5".to_string(),
            "novita" => "Qwen/Qwen3.5-35B-A3B".to_string(),
            "github-copilot" | "github-copilot-enterprise" => "gpt-5-mini".to_string(),
            _ => "gpt-4o".to_string(),
        }
    }

    /// Create a new session
    pub async fn new() -> Result<Self> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now();
        let provenance = Some(ExecutionProvenance::for_session(&id, "build"));

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
                provenance,
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

    pub fn set_agent_name(&mut self, agent_name: impl Into<String>) {
        let agent_name = agent_name.into();
        self.agent = agent_name.clone();
        if let Some(provenance) = self.metadata.provenance.as_mut() {
            provenance.set_agent_name(&agent_name);
        }
    }

    pub fn attach_worker_task_provenance(&mut self, worker_id: &str, task_id: &str) {
        if let Some(provenance) = self.metadata.provenance.as_mut() {
            provenance.apply_worker_task(worker_id, task_id);
        }
    }

    pub fn attach_claim_provenance(&mut self, claim: &ClaimProvenance) {
        if let Some(provenance) = self.metadata.provenance.as_mut() {
            provenance.apply_claim(claim);
        }
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

    async fn compress_history_keep_last(
        &mut self,
        provider: Arc<dyn crate::provider::Provider>,
        model: &str,
        keep_last: usize,
        reason: &str,
    ) -> Result<bool> {
        if self.messages.len() <= keep_last {
            return Ok(false);
        }

        let split_idx = self.messages.len().saturating_sub(keep_last);
        let tail = self.messages.split_off(split_idx);
        let prefix = std::mem::take(&mut self.messages);

        let context = messages_to_rlm_context(&prefix);
        let ctx_window = context_window_for_model(model);

        // We call auto_process directly for session context. It internally
        // compresses very large inputs before hitting the model.
        let rlm_config = RlmConfig::default();
        let auto_ctx = AutoProcessContext {
            tool_id: "session_context",
            tool_args: serde_json::json!({"reason": reason}),
            session_id: &self.id,
            abort: None,
            on_progress: None,
            provider,
            model: model.to_string(),
        };

        let summary = match RlmRouter::auto_process(&context, auto_ctx, &rlm_config).await {
            Ok(result) => {
                tracing::info!(
                    reason,
                    input_tokens = result.stats.input_tokens,
                    output_tokens = result.stats.output_tokens,
                    compression_ratio = result.stats.compression_ratio,
                    "RLM: Compressed session history"
                );
                result.processed
            }
            Err(e) => {
                tracing::warn!(reason, error = %e, "RLM: Failed to compress session history; falling back to chunk compression");
                // Fallback: keep a smaller, semantically chunked excerpt.
                RlmChunker::compress(&context, (ctx_window as f64 * 0.25) as usize, None)
            }
        };

        let summary_msg = Message {
            role: Role::Assistant,
            content: vec![ContentPart::Text {
                text: format!(
                    "[AUTO CONTEXT COMPRESSION]\nOlder conversation + tool output was compressed to fit the model context window.\n\n{}",
                    summary
                ),
            }],
        };

        let mut new_messages = Vec::with_capacity(1 + tail.len());
        new_messages.push(summary_msg);
        new_messages.extend(tail);
        self.messages = new_messages;
        self.updated_at = Utc::now();

        Ok(true)
    }

    async fn enforce_context_window(
        &mut self,
        provider: Arc<dyn crate::provider::Provider>,
        model: &str,
        system_prompt: &str,
        tools: &[ToolDefinition],
    ) -> Result<()> {
        let ctx_window = context_window_for_model(model);

        // Reserve response tokens + a small fixed overhead for tool schemas,
        // protocol framing, and provider-specific wrappers.
        let reserve = session_completion_max_tokens().saturating_add(2048);
        let budget = ctx_window.saturating_sub(reserve);
        let safety_budget = (budget as f64 * 0.90) as usize;

        // Try progressively more aggressive compression.
        let keep_last_candidates = [16usize, 12, 8, 6];
        for keep_last in keep_last_candidates {
            let est = estimate_request_tokens(system_prompt, &self.messages, tools);
            if est <= safety_budget {
                return Ok(());
            }

            tracing::info!(
                est_tokens = est,
                ctx_window,
                safety_budget,
                keep_last,
                "Context window approaching limit; compressing older session history"
            );

            let did = self
                .compress_history_keep_last(
                    Arc::clone(&provider),
                    model,
                    keep_last,
                    "context_budget",
                )
                .await?;

            if !did {
                // Nothing left to compress.
                break;
            }
        }

        Ok(())
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
                "No providers available. Configure provider credentials in HashiCorp Vault (for ChatGPT subscription Codex use `codetether auth codex`; for Copilot use `codetether auth copilot`)."
            );
        }

        tracing::info!("Available providers: {:?}", providers);

        // Parse model string (format: "provider/model", "provider", or just "model")
        let (provider_name, model_id) = if let Some(ref model_str) = self.metadata.model {
            let (prov, model) = parse_model_string(model_str);
            let prov = prov.map(|p| match p {
                "zhipuai" | "z-ai" => "zai",
                other => other,
            });
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

        let selected_provider =
            resolve_provider_for_session_request(providers.as_slice(), provider_name.as_deref())?;

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

        // Most providers support native tool calling.  For providers that
        // don't (e.g. gemini-web), the FunctionGemma tool-call router will
        // convert text-only responses into structured tool calls.
        let model_supports_tools = !matches!(
            selected_provider,
            "gemini-web" | "local-cuda" | "local_cuda" | "localcuda"
        );
        let advertised_tool_definitions = if model_supports_tools {
            tool_definitions.clone()
        } else {
            vec![list_tools_bootstrap_definition()]
        };

        // Build system prompt with AGENTS.md
        let cwd = self
            .metadata
            .directory
            .clone()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
        let system_prompt = if is_local_cuda_provider(selected_provider) {
            local_cuda_light_system_prompt()
        } else {
            crate::agent::builtin::build_system_prompt(&cwd)
        };

        // For models that don't support native tool calling, inject tool
        // definitions into the system prompt so the model outputs <tool_call>
        // XML blocks that the router can parse directly.
        let system_prompt = if !model_supports_tools && !advertised_tool_definitions.is_empty() {
            inject_tool_prompt(&system_prompt, &advertised_tool_definitions)
        } else {
            system_prompt
        };

        // Run agentic loop with tool execution
        let max_steps = 50;
        let mut final_output = String::new();
        let baseline_git_dirty_files = capture_git_dirty_files(&cwd).await;
        let mut touched_files = HashSet::new();
        let mut validation_retry_count: u8 = 0;

        // Track consecutive identical tool calls to detect infinite loops.
        let mut last_tool_sig: Option<String> = None;
        let mut consecutive_same_tool: u32 = 0;
        let mut consecutive_codesearch_no_matches: u32 = 0;
        let mut build_mode_tool_retry_count: u8 = 0;
        let mut native_tool_promise_retry_count: u8 = 0;
        const MAX_CONSECUTIVE_SAME_TOOL: u32 = 3;

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

            // Proactively keep the prompt within the model's context window.
            self.enforce_context_window(
                Arc::clone(&provider),
                &model,
                &system_prompt,
                &advertised_tool_definitions,
            )
            .await?;

            let proactive_lsp_message = build_proactive_lsp_context_message(
                selected_provider,
                step,
                &tool_registry,
                &self.messages,
                &cwd,
            )
            .await;

            // Call the provider (retry once if the provider rejects due to an
            // unexpected context-length mismatch).
            let mut attempt = 0;
            let response = loop {
                attempt += 1;

                // Build messages with system prompt first
                let mut messages = vec![Message {
                    role: Role::System,
                    content: vec![ContentPart::Text {
                        text: system_prompt.clone(),
                    }],
                }];
                if let Some(msg) = &proactive_lsp_message {
                    messages.push(msg.clone());
                }
                messages.extend(self.messages.clone());

                // Create completion request with tools
                let request = CompletionRequest {
                    messages,
                    tools: advertised_tool_definitions.clone(),
                    model: model.clone(),
                    temperature,
                    top_p: None,
                    max_tokens: Some(session_completion_max_tokens()),
                    stop: Vec::new(),
                };

                match provider.complete(request).await {
                    Ok(r) => break r,
                    Err(e) => {
                        if attempt == 1 && is_prompt_too_long_error(&e) {
                            tracing::warn!(error = %e, "Provider rejected prompt as too long; forcing extra compression and retrying");
                            let _ = self
                                .compress_history_keep_last(
                                    Arc::clone(&provider),
                                    &model,
                                    6,
                                    "prompt_too_long_retry",
                                )
                                .await?;
                            continue;
                        }
                        if attempt == 1 && is_retryable_upstream_error(&e) {
                            if let Some((retry_provider, retry_model)) =
                                choose_router_target(&registry, selected_provider, &model)
                            {
                                tracing::warn!(
                                    error = %e,
                                    from_provider = selected_provider,
                                    from_model = %model,
                                    to_provider = %retry_provider,
                                    to_model = %retry_model,
                                    "Retryable upstream provider error; retrying same prompt with alternate provider/model"
                                );
                                self.metadata.model =
                                    Some(format!("{retry_provider}/{retry_model}"));
                                attempt = 0;
                                continue;
                            }
                        }
                        return Err(e);
                    }
                }
            };

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
            let response = normalize_textual_tool_calls(response, &tool_definitions);

            // Record token usage
            crate::telemetry::TOKEN_USAGE.record_model_usage(
                &model,
                response.usage.prompt_tokens as u64,
                response.usage.completion_tokens as u64,
            );

            // Extract tool calls from response
            let mut truncated_tool_ids: Vec<(String, String)> = Vec::new();
            let tool_calls: Vec<(String, String, serde_json::Value)> = response
                .message
                .content
                .iter()
                .filter_map(|part| {
                    if let ContentPart::ToolCall {
                        id,
                        name,
                        arguments,
                        ..
                    } = part
                    {
                        match serde_json::from_str::<serde_json::Value>(arguments) {
                            Ok(args) => Some((id.clone(), name.clone(), args)),
                            Err(e) => {
                                tracing::warn!(
                                    tool = %name,
                                    tool_call_id = %id,
                                    args_len = arguments.len(),
                                    error = %e,
                                    "Tool call arguments failed to parse (likely truncated by max_tokens)"
                                );
                                truncated_tool_ids.push((id.clone(), name.clone()));
                                None
                            }
                        }
                    } else {
                        None
                    }
                })
                .collect();

            let assistant_text = extract_text_content(&&response.message.content);
            if should_force_build_tool_first_retry(
                &self.agent,
                build_mode_tool_retry_count,
                &tool_definitions,
                &self.messages,
                &cwd,
                &assistant_text,
                !tool_calls.is_empty(),
                BUILD_MODE_TOOL_FIRST_MAX_RETRIES,
            ) {
                build_mode_tool_retry_count += 1;
                tracing::warn!(
                    step = step,
                    agent = %self.agent,
                    retry = build_mode_tool_retry_count,
                    "Build mode tool-first guard triggered; retrying with execution nudge"
                );
                self.add_message(Message {
                    role: Role::User,
                    content: vec![ContentPart::Text {
                        text: BUILD_MODE_TOOL_FIRST_NUDGE.to_string(),
                    }],
                });
                continue;
            }
            if should_retry_missing_native_tool_call(
                selected_provider,
                &model,
                native_tool_promise_retry_count,
                &tool_definitions,
                &assistant_text,
                !tool_calls.is_empty(),
                NATIVE_TOOL_PROMISE_RETRY_MAX_RETRIES,
            ) {
                native_tool_promise_retry_count += 1;
                tracing::warn!(
                    step = step,
                    provider = selected_provider,
                    model = %model,
                    retry = native_tool_promise_retry_count,
                    "Model described a tool step without emitting a tool call; retrying with corrective nudge"
                );
                self.add_message(response.message.clone());
                self.add_message(Message {
                    role: Role::User,
                    content: vec![ContentPart::Text {
                        text: NATIVE_TOOL_PROMISE_NUDGE.to_string(),
                    }],
                });
                continue;
            }
            if !tool_calls.is_empty() {
                build_mode_tool_retry_count = 0;
                native_tool_promise_retry_count = 0;
            } else if is_build_agent(&self.agent)
                && build_request_requires_tool(&self.messages, &cwd)
                && build_mode_tool_retry_count >= BUILD_MODE_TOOL_FIRST_MAX_RETRIES
            {
                return Err(anyhow::anyhow!(
                    "Build mode could not obtain tool calls for an explicit file-change request after {} retries. \
                     Switch to a tool-capable model and try again.",
                    BUILD_MODE_TOOL_FIRST_MAX_RETRIES
                ));
            }

            let mut step_text = String::new();

            // Collect text output and publish thinking to bus
            for part in &response.message.content {
                match part {
                    ContentPart::Text { text } if !text.is_empty() => {
                        step_text.push_str(text);
                        step_text.push('\n');
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

            if !step_text.trim().is_empty() {
                final_output.push_str(&step_text);
            }

            // If no tool calls, we're done
            if tool_calls.is_empty() && truncated_tool_ids.is_empty() {
                self.add_message(response.message.clone());
                if is_build_agent(&self.agent) {
                    if let Some(report) =
                        build_validation_report(&cwd, &touched_files, &baseline_git_dirty_files)
                            .await?
                    {
                        validation_retry_count += 1;
                        tracing::warn!(
                            retries = validation_retry_count,
                            issues = report.issue_count,
                            "Post-edit validation found unresolved diagnostics"
                        );
                        if validation_retry_count > POST_EDIT_VALIDATION_MAX_RETRIES {
                            return Err(anyhow::anyhow!(
                                "Post-edit validation failed after {} attempts.\n\n{}",
                                POST_EDIT_VALIDATION_MAX_RETRIES,
                                report.prompt
                            ));
                        }
                        self.add_message(Message {
                            role: Role::User,
                            content: vec![ContentPart::Text {
                                text: report.prompt,
                            }],
                        });
                        final_output.clear();
                        continue;
                    }
                }
                break;
            }

            // Handle truncated tool calls: send error results back so the LLM can retry
            if !truncated_tool_ids.is_empty() {
                if tool_calls.is_empty() {
                    self.add_message(response.message.clone());
                }
                for (tool_id, tool_name) in &truncated_tool_ids {
                    let error_content = format!(
                        "Error: Your tool call to `{tool_name}` was truncated — the arguments \
                         JSON was cut off mid-string (likely hit the max_tokens limit). \
                         Please retry with a shorter approach: use the `write` tool to write \
                         content in smaller pieces, or reduce the size of your arguments."
                    );
                    self.add_message(Message {
                        role: Role::Tool,
                        content: vec![ContentPart::ToolResult {
                            tool_call_id: tool_id.clone(),
                            content: error_content,
                        }],
                    });
                }
                if tool_calls.is_empty() {
                    continue;
                }
            }

            // ── Loop detection: break if the same tool+args repeats too many times,
            //    or if a non-native-tool model has used too many consecutive tool steps. ──
            {
                // Signature = sorted list of "name:args" for deterministic comparison.
                let mut sigs: Vec<String> = tool_calls
                    .iter()
                    .map(|(_, name, args)| format!("{name}:{args}"))
                    .collect();
                sigs.sort();
                let sig = sigs.join("|");

                if last_tool_sig.as_deref() == Some(&sig) {
                    consecutive_same_tool += 1;
                } else {
                    consecutive_same_tool = 1;
                    last_tool_sig = Some(sig);
                }

                // For non-native-tool models, also guard against total tool steps.
                // These models tend to always include <tool_call> blocks and never
                // converge to a text-only answer on their own.
                let force_answer = consecutive_same_tool > MAX_CONSECUTIVE_SAME_TOOL
                    || (!model_supports_tools && step >= 3);

                if force_answer {
                    tracing::warn!(
                        step = step,
                        consecutive = consecutive_same_tool,
                        "Breaking agent loop: forcing final answer",
                    );
                    // Strip ToolCall parts from the response so the model
                    // doesn't see dangling calls without results.
                    let mut nudge_msg = response.message.clone();
                    nudge_msg
                        .content
                        .retain(|p| !matches!(p, ContentPart::ToolCall { .. }));
                    if !nudge_msg.content.is_empty() {
                        self.add_message(nudge_msg);
                    }
                    self.add_message(Message {
                        role: Role::User,
                        content: vec![ContentPart::Text {
                            text: "STOP using tools. Provide your final answer NOW \
                                   in plain text based on the tool results you already \
                                   received. Do NOT output any <tool_call> blocks."
                                .to_string(),
                        }],
                    });
                    continue;
                }
            }

            // Add assistant message with tool calls
            self.add_message(response.message.clone());

            tracing::info!(
                step = step,
                num_tools = tool_calls.len(),
                "Executing tool calls"
            );

            // Execute each tool call
            let mut codesearch_thrash_guard_triggered = false;
            for (tool_id, tool_name, tool_input) in tool_calls {
                let (tool_name, tool_input) =
                    normalize_tool_call_for_execution(&tool_name, &tool_input);
                tracing::info!(tool = %tool_name, tool_id = %tool_id, "Executing tool");

                if tool_name == "list_tools" {
                    let content = list_tools_bootstrap_output(&tool_definitions, &tool_input);
                    self.add_message(Message {
                        role: Role::Tool,
                        content: vec![ContentPart::ToolResult {
                            tool_call_id: tool_id,
                            content,
                        }],
                    });
                    continue;
                }

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
                            step,
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

                if let Some(reason) = detect_stub_in_tool_input(&tool_name, &tool_input) {
                    tracing::warn!(tool = %tool_name, reason = %reason, "Blocking suspected stubbed edit");
                    self.add_message(Message {
                        role: Role::Tool,
                        content: vec![ContentPart::ToolResult {
                            tool_call_id: tool_id,
                            content: format!(
                                "Error: Refactor guard rejected this edit: {reason}. \
                                 Provide concrete, behavior-preserving implementation (no placeholders/stubs)."
                            ),
                        }],
                    });
                    continue;
                }

                // Get and execute the tool
                let exec_start = std::time::Instant::now();
                let exec_input = enrich_tool_input_with_runtime_context(
                    &tool_input,
                    self.metadata.model.as_deref(),
                    &self.id,
                    &self.agent,
                    self.metadata.provenance.as_ref(),
                );
                let (content, success, tool_metadata) = if let Some(tool) =
                    tool_registry.get(&tool_name)
                {
                    match tool.execute(exec_input).await {
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
                            (result.output, result.success, Some(result.metadata))
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
                            (format!("Error: {}", e), false, None)
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
                    (format!("Error: Unknown tool '{}'", tool_name), false, None)
                };

                let requires_confirmation =
                    tool_result_requires_confirmation(tool_metadata.as_ref());
                let (content, success, tool_metadata, requires_confirmation) =
                    if requires_confirmation && self.metadata.auto_apply_edits {
                        let preview_content = content.clone();
                        match auto_apply_pending_confirmation(
                            &tool_name,
                            &tool_input,
                            tool_metadata.as_ref(),
                        )
                        .await
                        {
                            Ok(Some((content, success, tool_metadata))) => {
                                tracing::info!(
                                    tool = %tool_name,
                                    "Auto-applied pending confirmation in TUI session"
                                );
                                (content, success, tool_metadata, false)
                            }
                            Ok(None) => (content, success, tool_metadata, true),
                            Err(error) => (
                                format!(
                                    "{}\n\nTUI edit auto-apply failed: {}",
                                    pending_confirmation_tool_result_content(
                                        &tool_name,
                                        &preview_content,
                                    ),
                                    error
                                ),
                                false,
                                tool_metadata,
                                true,
                            ),
                        }
                    } else {
                        (content, success, tool_metadata, requires_confirmation)
                    };
                let rendered_content = if requires_confirmation {
                    pending_confirmation_tool_result_content(&tool_name, &content)
                } else {
                    content.clone()
                };

                if !requires_confirmation {
                    track_touched_files(
                        &mut touched_files,
                        &cwd,
                        &tool_name,
                        &tool_input,
                        tool_metadata.as_ref(),
                    );
                }

                // Calculate duration for event stream
                let duration_ms = exec_start.elapsed().as_millis() as u64;
                let codesearch_no_match =
                    is_codesearch_no_match_output(&tool_name, success, &rendered_content);

                // Publish tool response + full tool output to bus for training pipeline
                if let Some(ref bus) = self.bus {
                    let handle = bus.handle(&self.agent);
                    handle.send(
                        format!("agent.{}.tool.response", self.agent),
                        crate::bus::BusMessage::ToolResponse {
                            request_id: tool_id.clone(),
                            agent_id: self.agent.clone(),
                            tool_name: tool_name.clone(),
                            result: rendered_content.clone(),
                            success,
                            step,
                        },
                    );
                    // (before RLM truncation so we capture the complete output)
                    handle.send(
                        format!("agent.{}.tool.output", self.agent),
                        crate::bus::BusMessage::ToolOutputFull {
                            agent_id: self.agent.clone(),
                            tool_name: tool_name.clone(),
                            output: rendered_content.clone(),
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
                        &rendered_content,
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
                    let current_tokens = estimate_tokens_for_messages(&self.messages);
                    let routing_ctx = RoutingContext {
                        tool_id: tool_name.clone(),
                        session_id: self.id.clone(),
                        call_id: Some(tool_id.clone()),
                        model_context_limit: ctx_window,
                        current_context_tokens: Some(current_tokens),
                    };
                    let rlm_config = RlmConfig::default();
                    let routing =
                        RlmRouter::should_route(&rendered_content, &routing_ctx, &rlm_config);
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
                        match RlmRouter::auto_process(&rendered_content, auto_ctx, &rlm_config)
                            .await
                        {
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
                                    &rendered_content,
                                    &tool_name,
                                    &tool_input,
                                    ctx_window / 4,
                                );
                                truncated
                            }
                        }
                    } else {
                        rendered_content
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

                if is_build_agent(&self.agent) {
                    if codesearch_no_match {
                        consecutive_codesearch_no_matches += 1;
                    } else {
                        consecutive_codesearch_no_matches = 0;
                    }

                    if consecutive_codesearch_no_matches >= MAX_CONSECUTIVE_CODESEARCH_NO_MATCHES {
                        tracing::warn!(
                            step = step,
                            consecutive_codesearch_no_matches = consecutive_codesearch_no_matches,
                            "Detected codesearch no-match thrash; nudging model to stop variant retries",
                        );
                        self.add_message(Message {
                            role: Role::User,
                            content: vec![ContentPart::Text {
                                text: CODESEARCH_THRASH_NUDGE.to_string(),
                            }],
                        });
                        codesearch_thrash_guard_triggered = true;
                        break;
                    }
                }
            }

            if codesearch_thrash_guard_triggered {
                continue;
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
        self.prompt_with_events_and_images(message, Vec::new(), event_tx, registry)
            .await
    }

    /// Execute a prompt with optional image attachments and stream events.
    ///
    /// This is the full-featured version that supports multimodal input (text + images).
    /// Images should be base64-encoded data URLs.
    pub async fn prompt_with_events_and_images(
        &mut self,
        message: &str,
        images: Vec<ImageAttachment>,
        event_tx: tokio::sync::mpsc::Sender<SessionEvent>,
        registry: std::sync::Arc<crate::provider::ProviderRegistry>,
    ) -> Result<SessionResult> {
        use crate::provider::{CompletionRequest, ContentPart, Role, parse_model_string};

        let _ = event_tx.send(SessionEvent::Thinking).await;

        let providers = registry.list();
        if providers.is_empty() {
            anyhow::bail!(
                "No providers available. Configure provider credentials in HashiCorp Vault (for ChatGPT subscription Codex use `codetether auth codex`; for Copilot use `codetether auth copilot`)."
            );
        }
        tracing::info!("Available providers: {:?}", providers);

        // Parse model string (format: "provider/model", "provider", or just "model")
        let (provider_name, model_id) = if let Some(ref model_str) = self.metadata.model {
            let (prov, model) = parse_model_string(model_str);
            let prov = prov.map(|p| match p {
                "zhipuai" | "z-ai" => "zai",
                other => other,
            });
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

        let selected_provider =
            resolve_provider_for_session_request(providers.as_slice(), provider_name.as_deref())?;

        let provider = registry
            .get(selected_provider)
            .ok_or_else(|| anyhow::anyhow!("Provider {} not found", selected_provider))?;

        // Build user message content parts (text + optional images)
        let mut content_parts = vec![ContentPart::Text {
            text: message.to_string(),
        }];

        // Add image attachments
        for img in &images {
            content_parts.push(ContentPart::Image {
                url: img.data_url.clone(),
                mime_type: img.mime_type.clone(),
            });
        }

        if !images.is_empty() {
            tracing::info!(
                image_count = images.len(),
                "Adding {} image attachment(s) to user message",
                images.len()
            );
        }

        // Add user message
        self.add_message(Message {
            role: Role::User,
            content: content_parts,
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

        // Most providers support native tool calling.  For providers that
        // don't (e.g. gemini-web), the FunctionGemma tool-call router will
        // convert text-only responses into structured tool calls.
        let model_supports_tools = !matches!(
            selected_provider,
            "gemini-web" | "local-cuda" | "local_cuda" | "localcuda"
        );
        let advertised_tool_definitions = if model_supports_tools {
            tool_definitions.clone()
        } else {
            vec![list_tools_bootstrap_definition()]
        };

        // Build system prompt
        let cwd = std::env::var("PWD")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|_| std::env::current_dir().unwrap_or_default());
        let system_prompt = if is_local_cuda_provider(selected_provider) {
            local_cuda_light_system_prompt()
        } else {
            crate::agent::builtin::build_system_prompt(&cwd)
        };

        // For models that don't support native tool calling, inject tool
        // definitions into the system prompt.
        let system_prompt = if !model_supports_tools && !advertised_tool_definitions.is_empty() {
            inject_tool_prompt(&system_prompt, &advertised_tool_definitions)
        } else {
            system_prompt
        };

        let mut final_output = String::new();
        let max_steps = 50;
        let baseline_git_dirty_files = capture_git_dirty_files(&cwd).await;
        let mut touched_files = HashSet::new();
        let mut validation_retry_count: u8 = 0;

        // Track consecutive identical tool calls to detect infinite loops.
        let mut last_tool_sig: Option<String> = None;
        let mut consecutive_same_tool: u32 = 0;
        let mut consecutive_codesearch_no_matches: u32 = 0;
        let mut build_mode_tool_retry_count: u8 = 0;
        let mut native_tool_promise_retry_count: u8 = 0;
        const MAX_CONSECUTIVE_SAME_TOOL: u32 = 3;

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

            // Proactively keep the prompt within the model's context window.
            self.enforce_context_window(
                Arc::clone(&provider),
                &model,
                &system_prompt,
                &advertised_tool_definitions,
            )
            .await?;

            let proactive_lsp_message = build_proactive_lsp_context_message(
                selected_provider,
                step,
                &tool_registry,
                &self.messages,
                &cwd,
            )
            .await;

            // Build messages with system prompt first
            let mut messages = vec![Message {
                role: Role::System,
                content: vec![ContentPart::Text {
                    text: system_prompt.clone(),
                }],
            }];
            if let Some(msg) = &proactive_lsp_message {
                messages.push(msg.clone());
            }
            messages.extend(self.messages.clone());

            let request = CompletionRequest {
                messages,
                tools: advertised_tool_definitions.clone(),
                model: model.clone(),
                temperature,
                top_p: None,
                max_tokens: Some(session_completion_max_tokens()),
                stop: Vec::new(),
            };

            let llm_start = std::time::Instant::now();
            let mut attempt = 0;
            #[allow(clippy::never_loop)]
            let response = loop {
                attempt += 1;
                let completion_result = if model_supports_tools {
                    let stream = provider.complete_stream(request.clone()).await?;
                    collect_stream_completion_with_events(stream, Some(&event_tx)).await
                } else {
                    provider.complete(request.clone()).await
                };

                match completion_result {
                    Ok(r) => break r,
                    Err(e) => {
                        if attempt == 1 && is_prompt_too_long_error(&e) {
                            tracing::warn!(error = %e, "Provider rejected prompt as too long; forcing extra compression and retrying");
                            let _ = self
                                .compress_history_keep_last(
                                    Arc::clone(&provider),
                                    &model,
                                    6,
                                    "prompt_too_long_retry",
                                )
                                .await?;
                            // Rebuild request with the newly-compressed history.
                            // NOTE: we can't mutate the existing request in place; it contains
                            // cloned message vectors.
                            let mut messages = vec![Message {
                                role: Role::System,
                                content: vec![ContentPart::Text {
                                    text: system_prompt.clone(),
                                }],
                            }];
                            if let Some(msg) = &proactive_lsp_message {
                                messages.push(msg.clone());
                            }
                            messages.extend(self.messages.clone());
                            let new_request = CompletionRequest {
                                messages,
                                tools: advertised_tool_definitions.clone(),
                                model: model.clone(),
                                temperature,
                                top_p: None,
                                max_tokens: Some(session_completion_max_tokens()),
                                stop: Vec::new(),
                            };
                            let retry_result = if model_supports_tools {
                                let stream = provider.complete_stream(new_request).await?;
                                collect_stream_completion_with_events(stream, Some(&event_tx)).await
                            } else {
                                provider.complete(new_request).await
                            };
                            match retry_result {
                                Ok(r2) => break r2,
                                Err(e2) => return Err(e2),
                            }
                        }
                        return Err(e);
                    }
                }
            };
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
            let response = normalize_textual_tool_calls(response, &tool_definitions);

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
            let mut truncated_tool_ids: Vec<(String, String)> = Vec::new();
            let tool_calls: Vec<(String, String, serde_json::Value)> = response
                .message
                .content
                .iter()
                .filter_map(|part| {
                    if let ContentPart::ToolCall {
                        id,
                        name,
                        arguments,
                        ..
                    } = part
                    {
                        match serde_json::from_str::<serde_json::Value>(arguments) {
                            Ok(args) => Some((id.clone(), name.clone(), args)),
                            Err(e) => {
                                tracing::warn!(
                                    tool = %name,
                                    tool_call_id = %id,
                                    args_len = arguments.len(),
                                    error = %e,
                                    "Tool call arguments failed to parse (likely truncated by max_tokens)"
                                );
                                truncated_tool_ids.push((id.clone(), name.clone()));
                                None
                            }
                        }
                    } else {
                        None
                    }
                })
                .collect();

            let assistant_text = extract_text_content(&&response.message.content);
            if should_force_build_tool_first_retry(
                &self.agent,
                build_mode_tool_retry_count,
                &tool_definitions,
                &self.messages,
                &cwd,
                &assistant_text,
                !tool_calls.is_empty(),
                BUILD_MODE_TOOL_FIRST_MAX_RETRIES,
            ) {
                build_mode_tool_retry_count += 1;
                tracing::warn!(
                    step = step,
                    agent = %self.agent,
                    retry = build_mode_tool_retry_count,
                    "Build mode tool-first guard triggered; retrying with execution nudge"
                );
                self.add_message(Message {
                    role: Role::User,
                    content: vec![ContentPart::Text {
                        text: BUILD_MODE_TOOL_FIRST_NUDGE.to_string(),
                    }],
                });
                continue;
            }
            if should_retry_missing_native_tool_call(
                selected_provider,
                &model,
                native_tool_promise_retry_count,
                &tool_definitions,
                &assistant_text,
                !tool_calls.is_empty(),
                NATIVE_TOOL_PROMISE_RETRY_MAX_RETRIES,
            ) {
                native_tool_promise_retry_count += 1;
                tracing::warn!(
                    step = step,
                    provider = selected_provider,
                    model = %model,
                    retry = native_tool_promise_retry_count,
                    "Model described a tool step without emitting a tool call; retrying with corrective nudge"
                );
                self.add_message(response.message.clone());
                self.add_message(Message {
                    role: Role::User,
                    content: vec![ContentPart::Text {
                        text: NATIVE_TOOL_PROMISE_NUDGE.to_string(),
                    }],
                });
                continue;
            }
            if !tool_calls.is_empty() {
                build_mode_tool_retry_count = 0;
                native_tool_promise_retry_count = 0;
            } else if is_build_agent(&self.agent)
                && build_request_requires_tool(&self.messages, &cwd)
                && build_mode_tool_retry_count >= BUILD_MODE_TOOL_FIRST_MAX_RETRIES
            {
                return Err(anyhow::anyhow!(
                    "Build mode could not obtain tool calls for an explicit file-change request after {} retries. \
                     Switch to a tool-capable model and try again.",
                    BUILD_MODE_TOOL_FIRST_MAX_RETRIES
                ));
            }

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

            if tool_calls.is_empty() && truncated_tool_ids.is_empty() {
                self.add_message(response.message.clone());
                if is_build_agent(&self.agent) {
                    if let Some(report) =
                        build_validation_report(&cwd, &touched_files, &baseline_git_dirty_files)
                            .await?
                    {
                        validation_retry_count += 1;
                        tracing::warn!(
                            retries = validation_retry_count,
                            issues = report.issue_count,
                            "Post-edit validation found unresolved diagnostics"
                        );
                        if validation_retry_count > POST_EDIT_VALIDATION_MAX_RETRIES {
                            return Err(anyhow::anyhow!(
                                "Post-edit validation failed after {} attempts.\n\n{}",
                                POST_EDIT_VALIDATION_MAX_RETRIES,
                                report.prompt
                            ));
                        }
                        self.add_message(Message {
                            role: Role::User,
                            content: vec![ContentPart::Text {
                                text: report.prompt,
                            }],
                        });
                        final_output.clear();
                        continue;
                    }
                }
                break;
            }

            // Handle truncated tool calls: send error results back so the LLM can retry
            if !truncated_tool_ids.is_empty() {
                if tool_calls.is_empty() {
                    self.add_message(response.message.clone());
                }
                for (tool_id, tool_name) in &truncated_tool_ids {
                    let error_content = format!(
                        "Error: Your tool call to `{tool_name}` was truncated — the arguments \
                         JSON was cut off mid-string (likely hit the max_tokens limit). \
                         Please retry with a shorter approach: use the `write` tool to write \
                         content in smaller pieces, or reduce the size of your arguments."
                    );
                    let _ = event_tx
                        .send(SessionEvent::ToolCallComplete {
                            name: tool_name.clone(),
                            output: error_content.clone(),
                            success: false,
                            duration_ms: 0,
                        })
                        .await;
                    self.add_message(Message {
                        role: Role::Tool,
                        content: vec![ContentPart::ToolResult {
                            tool_call_id: tool_id.clone(),
                            content: error_content,
                        }],
                    });
                }
                if tool_calls.is_empty() {
                    continue;
                }
            }

            // ── Loop detection: break if the same tool+args repeats too many times,
            //    or if a non-native-tool model has used too many consecutive tool steps. ──
            {
                let mut sigs: Vec<String> = tool_calls
                    .iter()
                    .map(|(_, name, args)| format!("{name}:{args}"))
                    .collect();
                sigs.sort();
                let sig = sigs.join("|");

                if last_tool_sig.as_deref() == Some(&sig) {
                    consecutive_same_tool += 1;
                } else {
                    consecutive_same_tool = 1;
                    last_tool_sig = Some(sig);
                }

                let force_answer = consecutive_same_tool > MAX_CONSECUTIVE_SAME_TOOL
                    || (!model_supports_tools && step >= 3);

                if force_answer {
                    tracing::warn!(
                        step = step,
                        consecutive = consecutive_same_tool,
                        "Breaking agent loop: forcing final answer",
                    );
                    let mut nudge_msg = response.message.clone();
                    nudge_msg
                        .content
                        .retain(|p| !matches!(p, ContentPart::ToolCall { .. }));
                    if !nudge_msg.content.is_empty() {
                        self.add_message(nudge_msg);
                    }
                    self.add_message(Message {
                        role: Role::User,
                        content: vec![ContentPart::Text {
                            text: "STOP using tools. Provide your final answer NOW \
                                   in plain text based on the tool results you already \
                                   received. Do NOT output any <tool_call> blocks."
                                .to_string(),
                        }],
                    });
                    continue;
                }
            }

            self.add_message(response.message.clone());

            tracing::info!(
                step = step,
                num_tools = tool_calls.len(),
                "Executing tool calls"
            );

            // Execute each tool call with events
            let mut codesearch_thrash_guard_triggered = false;
            for (tool_id, tool_name, tool_input) in tool_calls {
                let (tool_name, tool_input) =
                    normalize_tool_call_for_execution(&tool_name, &tool_input);
                let args_str = serde_json::to_string(&tool_input).unwrap_or_default();
                let _ = event_tx
                    .send(SessionEvent::ToolCallStart {
                        name: tool_name.clone(),
                        arguments: args_str,
                    })
                    .await;

                tracing::info!(tool = %tool_name, tool_id = %tool_id, "Executing tool");

                if tool_name == "list_tools" {
                    let content = list_tools_bootstrap_output(&tool_definitions, &tool_input);
                    let _ = event_tx
                        .send(SessionEvent::ToolCallComplete {
                            name: tool_name.clone(),
                            output: content.clone(),
                            success: true,
                            duration_ms: 0,
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
                            step,
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
                            duration_ms: 0,
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

                if let Some(reason) = detect_stub_in_tool_input(&tool_name, &tool_input) {
                    tracing::warn!(tool = %tool_name, reason = %reason, "Blocking suspected stubbed edit");
                    let content = format!(
                        "Error: Refactor guard rejected this edit: {reason}. \
                         Provide concrete, behavior-preserving implementation (no placeholders/stubs)."
                    );
                    let _ = event_tx
                        .send(SessionEvent::ToolCallComplete {
                            name: tool_name.clone(),
                            output: content.clone(),
                            success: false,
                            duration_ms: 0,
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
                let exec_input = enrich_tool_input_with_runtime_context(
                    &tool_input,
                    self.metadata.model.as_deref(),
                    &self.id,
                    &self.agent,
                    self.metadata.provenance.as_ref(),
                );
                let (content, success, tool_metadata) = if let Some(tool) =
                    tool_registry.get(&tool_name)
                {
                    match tool.execute(exec_input).await {
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
                            (result.output, result.success, Some(result.metadata))
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
                            (format!("Error: {}", e), false, None)
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
                    (format!("Error: Unknown tool '{}'", tool_name), false, None)
                };

                let requires_confirmation =
                    tool_result_requires_confirmation(tool_metadata.as_ref());
                let (content, success, tool_metadata, requires_confirmation) =
                    if requires_confirmation && self.metadata.auto_apply_edits {
                        let preview_content = content.clone();
                        match auto_apply_pending_confirmation(
                            &tool_name,
                            &tool_input,
                            tool_metadata.as_ref(),
                        )
                        .await
                        {
                            Ok(Some((content, success, tool_metadata))) => {
                                tracing::info!(
                                    tool = %tool_name,
                                    "Auto-applied pending confirmation in TUI session"
                                );
                                (content, success, tool_metadata, false)
                            }
                            Ok(None) => (content, success, tool_metadata, true),
                            Err(error) => (
                                format!(
                                    "{}\n\nTUI edit auto-apply failed: {}",
                                    pending_confirmation_tool_result_content(
                                        &tool_name,
                                        &preview_content,
                                    ),
                                    error
                                ),
                                false,
                                tool_metadata,
                                true,
                            ),
                        }
                    } else {
                        (content, success, tool_metadata, requires_confirmation)
                    };
                let rendered_content = if requires_confirmation {
                    pending_confirmation_tool_result_content(&tool_name, &content)
                } else {
                    content.clone()
                };

                if !requires_confirmation {
                    track_touched_files(
                        &mut touched_files,
                        &cwd,
                        &tool_name,
                        &tool_input,
                        tool_metadata.as_ref(),
                    );
                }

                // Calculate total duration from exec_start (captured from line 772)
                let duration_ms = exec_start.elapsed().as_millis() as u64;
                let codesearch_no_match =
                    is_codesearch_no_match_output(&tool_name, success, &rendered_content);

                // Publish tool response + full tool output to bus for training pipeline
                if let Some(ref bus) = self.bus {
                    let handle = bus.handle(&self.agent);
                    handle.send(
                        format!("agent.{}.tool.response", self.agent),
                        crate::bus::BusMessage::ToolResponse {
                            request_id: tool_id.clone(),
                            agent_id: self.agent.clone(),
                            tool_name: tool_name.clone(),
                            result: rendered_content.clone(),
                            success,
                            step,
                        },
                    );
                    handle.send(
                        format!("agent.{}.tool.output", self.agent),
                        crate::bus::BusMessage::ToolOutputFull {
                            agent_id: self.agent.clone(),
                            tool_name: tool_name.clone(),
                            output: rendered_content.clone(),
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
                        &rendered_content,
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
                        output: rendered_content.clone(),
                        success,
                        duration_ms,
                    })
                    .await;

                // Route large tool outputs through RLM
                let content = {
                    let ctx_window = context_window_for_model(&model);
                    let current_tokens = estimate_tokens_for_messages(&self.messages);
                    let routing_ctx = RoutingContext {
                        tool_id: tool_name.clone(),
                        session_id: self.id.clone(),
                        call_id: Some(tool_id.clone()),
                        model_context_limit: ctx_window,
                        current_context_tokens: Some(current_tokens),
                    };
                    let rlm_config = RlmConfig::default();
                    let routing =
                        RlmRouter::should_route(&rendered_content, &routing_ctx, &rlm_config);
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
                        match RlmRouter::auto_process(&rendered_content, auto_ctx, &rlm_config)
                            .await
                        {
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
                                    &rendered_content,
                                    &tool_name,
                                    &tool_input,
                                    ctx_window / 4,
                                );
                                truncated
                            }
                        }
                    } else {
                        rendered_content
                    }
                };

                self.add_message(Message {
                    role: Role::Tool,
                    content: vec![ContentPart::ToolResult {
                        tool_call_id: tool_id,
                        content,
                    }],
                });

                if is_build_agent(&self.agent) {
                    if codesearch_no_match {
                        consecutive_codesearch_no_matches += 1;
                    } else {
                        consecutive_codesearch_no_matches = 0;
                    }

                    if consecutive_codesearch_no_matches >= MAX_CONSECUTIVE_CODESEARCH_NO_MATCHES {
                        tracing::warn!(
                            step = step,
                            consecutive_codesearch_no_matches = consecutive_codesearch_no_matches,
                            "Detected codesearch no-match thrash; nudging model to stop variant retries",
                        );
                        self.add_message(Message {
                            role: Role::User,
                            content: vec![ContentPart::Text {
                                text: CODESEARCH_THRASH_NUDGE.to_string(),
                            }],
                        });
                        codesearch_thrash_guard_triggered = true;
                        break;
                    }
                }
            }

            if codesearch_thrash_guard_triggered {
                continue;
            }
        }

        self.save().await?;

        // Archive event stream to S3/R2 if configured (for compliance: SOC 2, FedRAMP, ATO)
        self.archive_event_stream_to_s3().await;

        // Text was already sent per-step via TextComplete events.
        // Send updated session state so the caller can sync back.
        let _ = event_tx
            .send(SessionEvent::SessionSync(Box::new(self.clone())))
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
        duration_ms: u64,
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
    SessionSync(Box<Session>),
    /// Processing complete
    Done,
    /// Error occurred
    Error(String),
}

#[cfg(test)]
mod tests;
